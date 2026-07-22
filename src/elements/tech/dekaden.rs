use lotus_extra::vehicle::CockpitSide;
use lotus_script::time::delta;

use crate::api::{animation::Animation, key_event::KeyEvent, sound::Sound};

pub struct DecadeSwitchBuilder {
    cab_side: Option<CockpitSide>,

    pos: f32,
    target: f32,
    pre_target: f32,

    step_last: u8,
    new_step: u8,

    value: f32,
    max_value: u8,

    rotation_speed: f32,

    pos_anim: Animation,

    snd_dekade_press: Sound,
    snd_dekade_release: Sound,

    key_plus: KeyEvent,
    key_minus: KeyEvent,
}

impl DecadeSwitchBuilder {
    pub fn rotation_speed(mut self, rotation_speed: f32) -> Self {
        self.rotation_speed = rotation_speed;
        self
    }

    pub fn button_events(
        mut self,
        event_plus_name: impl Into<String>,
        event_minus_name: impl Into<String>,
    ) -> Self {
        self.key_plus = KeyEvent::new(Some(&event_plus_name.into()), self.cab_side);
        self.key_minus = KeyEvent::new(Some(&event_minus_name.into()), self.cab_side);
        self
    }

    pub fn add_btn_sound(
        mut self,
        snd_press_name: impl Into<String>,
        snd_release_name: impl Into<String>,
    ) -> Self {
        self.snd_dekade_press = Sound::new_simple(Some(&snd_press_name.into()));
        self.snd_dekade_release = Sound::new_simple(Some(&snd_release_name.into()));
        self
    }

    pub fn init_value(mut self, value: f32) -> Self {
        self.value = value;
        self.pos = value;
        self.target = value;
        self
    }

    pub fn build(self) -> DecadeSwitch {
        DecadeSwitch {
            cab_side: self.cab_side,
            pos: self.pos,
            target: self.target,
            pre_target: self.pre_target,
            running: false,
            value: self.value,
            step_last: self.step_last,
            new_step: self.new_step,
            max_value: self.max_value,
            rotation_speed: self.rotation_speed,
            pos_anim: self.pos_anim,
            snd_dekade_press: self.snd_dekade_press,
            snd_dekade_release: self.snd_dekade_release,
            key_plus: self.key_plus,
            key_minus: self.key_minus,
        }
    }
}

#[derive(Debug)]
pub struct DecadeSwitch {
    cab_side: Option<CockpitSide>,

    pub pos: f32,
    pub target: f32,
    pre_target: f32,

    pub running: bool,

    pub step_last: u8,
    pub new_step: u8,

    pub value: f32,
    max_value: u8,

    rotation_speed: f32,

    pos_anim: Animation,

    snd_dekade_press: Sound,
    snd_dekade_release: Sound,

    pub key_plus: KeyEvent,
    pub key_minus: KeyEvent,
}

/// Wenn |pos| diesen Vielfachen-Wert von max_value überschreitet, wird
/// pos/target/pre_target gemeinsam um ein Vielfaches von max_value verschoben,
/// um f32-Präzisionsverlust bei sehr langer Laufzeit zu vermeiden.
/// 10_000 volle Umdrehungen sind ein guter Kompromiss zwischen
/// "selten genug, um Carry-Logik nicht zu stören" und "f32 bleibt präzise".
const RECENTER_THRESHOLD_TURNS: f32 = 10_000.0;

impl DecadeSwitch {
    pub fn builder(
        max_value: u8,
        animation_name: impl Into<String>,
        cab_side: Option<CockpitSide>,
    ) -> DecadeSwitchBuilder {
        DecadeSwitchBuilder {
            cab_side,
            pos: 0.0,
            target: 0.0,
            pre_target: 0.0,

            value: 0.0,
            max_value,

            step_last: 0,
            new_step: 0,

            rotation_speed: 1.0,

            pos_anim: Animation::new(Some(&animation_name.into())),

            snd_dekade_press: Sound::new_simple(None),
            snd_dekade_release: Sound::new_simple(None),

            key_plus: KeyEvent::new(None, cab_side),
            key_minus: KeyEvent::new(None, cab_side),
        }
    }

    pub fn tick(&mut self, add_target: f32) -> f32 {
        let max_val = self.max_value as f32;

        if self.key_plus.is_just_pressed() || self.key_minus.is_just_pressed() {
            self.snd_dekade_press.start();
        }
        if self.key_plus.is_just_released() || self.key_minus.is_just_released() {
            self.snd_dekade_release.start();
        }

        if (self.pos - self.target).abs() < 0.001 {
            if self.key_plus.is_just_pressed() {
                self.target += 1.0;
            }

            if self.key_minus.is_just_pressed() {
                self.target -= 1.0;
            }

            // Anstehenden Carry/Eingabe-Überhang IMMER additiv verrechnen,
            // niemals target überschreiben - sonst gehen aufgelaufene
            // Übertragsimpulse verloren, wenn mehrere kurz hintereinander
            // eintreffen, während diese Dekade noch in Bewegung ist.
            self.target += self.pre_target;
            self.pre_target = 0.0;

            self.target += add_target;

            // Rundungskorrektur gegen f32-Restfehler (rein kosmetisch,
            // ändert den logischen Wert nicht)
            if (self.pos - self.pos.round()).abs() < 0.000001 {
                self.pos = self.pos.round();
            }
            if (self.target - self.target.round()).abs() < 0.000001 {
                self.target = self.target.round();
            }
        } else {
            // Während der Bewegung: anstehende Werte additiv sammeln.
            self.pre_target += add_target;
        }

        let pos_last = self.pos;

        if self.target > self.pos {
            self.pos = (self.pos + self.rotation_speed * delta()).min(self.target);
        } else {
            self.pos = (self.pos - self.rotation_speed * delta()).max(self.target);
        }

        // Anzeige/Animation bekommt immer den normalisierten Wert,
        // pos selbst bleibt unnormalisiert (siehe Kommentar an der Struktur).
        self.pos_anim.set(self.pos.rem_euclid(max_val));

        self.running = self.pos != self.target;

        self.value = self.pos.rem_euclid(max_val);

        // step_last/new_step als Anzeige-Hilfswerte aus der tatsächlichen
        // pos-Bewegung ableiten.
        self.update_step_display(pos_last, self.pos);

        // Carry über die geometrische Überlappung des in diesem Frame
        // zurückgelegten Bewegungssegments [pos_last, pos] mit der
        // "Kopplungszone" direkt vor jeder max_value-Schwelle ermitteln.
        // Das sorgt dafür, dass die Nachbarstelle PARALLEL zur Bewegung
        // mitdreht, sobald diese Dekade die Zone betritt - nicht erst,
        // wenn sie eine volle Umdrehung abgeschlossen hat.
        let carry = Self::zone_overlap_carry(pos_last, self.pos, max_val);

        // Sicherheitsmechanismus: pos/target/pre_target laufen bewusst
        // unnormalisiert (nicht mehr per rem_euclid zurückgefaltet), damit
        // mehrere kurz aufeinanderfolgende Carry-Ereignisse nicht
        // durcheinanderlaufen. Das heißt aber, die Werte wachsen über die
        // Laufzeit unbegrenzt. Nur wenn die Dekade gerade "ruht" (pos ==
        // target, kein offener pre_target) und weit genug von der nächsten
        // Schwelle entfernt ist, ziehen wir ein gemeinsames Vielfaches von
        // max_value ab. Das ändert nichts an Anzeige oder Carry-Verhalten,
        // hält die f32-Werte aber dauerhaft klein und präzise.
        self.recenter_if_needed();

        carry
    }

    /// Berechnet den Carry-Anteil für die Nachbarstelle aus der Überlappung
    /// des Bewegungssegments [pos_last, pos_new] mit den "Kopplungszonen"
    /// [n*max_val - 1, n*max_val) für jedes ganzzahlige n.
    ///
    /// Die Kopplungszone ist die letzte Einheit vor jeder vollen
    /// max_value-Schwelle (z. B. bei max_value=10 die Zone [9, 10), bzw.
    /// [19, 20), [-1, 0) usw.). Solange sich pos innerhalb dieser Zone
    /// bewegt, soll die Nachbarstelle 1:1 mitdrehen - so wie bei einem
    /// mechanischen Zahnrad, das schon kurz vor der vollen Umdrehung des
    /// Vorgängerrads beginnt mitzulaufen.
    ///
    /// Der Rückgabewert ist die Strecke (mit Vorzeichen der
    /// Bewegungsrichtung), die innerhalb dieser Zonen zurückgelegt wurde.
    /// Liegt das gesamte Segment außerhalb jeder Zone, ist das Ergebnis 0.
    /// Überquert ein einzelner Frame mehrere Zonen (z. B. bei einem sehr
    /// großen Einzelsprung), werden alle betroffenen Zonen aufsummiert.
    fn zone_overlap_carry(pos_last: f32, pos_new: f32, max_val: f32) -> f32 {
        if pos_new == pos_last {
            return 0.0;
        }

        let direction = if pos_new > pos_last { 1.0 } else { -1.0 };
        let lo = pos_last.min(pos_new);
        let hi = pos_last.max(pos_new);

        // Alle Zonen-Indizes n, deren Zone [n*max_val - 1, n*max_val) das
        // Segment [lo, hi] potenziell schneiden könnte.
        let n_start = (lo / max_val).floor() as i64 - 1;
        let n_end = (hi / max_val).floor() as i64 + 2;

        let mut total_overlap = 0.0;
        for n in n_start..=n_end {
            let zone_lo = n as f32 * max_val - 1.0;
            let zone_hi = n as f32 * max_val;

            let overlap_lo = lo.max(zone_lo);
            let overlap_hi = hi.min(zone_hi);

            if overlap_hi > overlap_lo {
                total_overlap += overlap_hi - overlap_lo;
            }
        }

        direction * total_overlap
    }

    fn recenter_if_needed(&mut self) {
        let max_val = self.max_value as f32;
        let threshold = RECENTER_THRESHOLD_TURNS * max_val;

        if self.pos.abs() < threshold {
            return;
        }

        // Nur re-centern, wenn die Dekade ruht und kein Übertrag mehr
        // ausstehend ist - sonst könnten wir mitten in einer Carry-Kette
        // den Bezugspunkt verschieben.
        if self.running || self.pre_target != 0.0 {
            return;
        }

        // Ganzes Vielfaches von max_value abziehen, damit pos im
        // Bereich [0, max_value) landet, target sich exakt mitverschiebt.
        let shift = (self.pos / max_val).floor() * max_val;

        if shift != 0.0 {
            self.pos -= shift;
            self.target -= shift;
        }
    }

    fn update_step_display(&mut self, pos_last: f32, new_pos: f32) {
        let max_val = self.max_value as f32;

        // Normalisierte Werte ausschließlich für die Anzeige von
        // step_last/new_step (0..max_value). Diese Werte haben keinen
        // Einfluss mehr auf die Carry-Erkennung (siehe tick()) - sie dienen
        // nur noch dazu, von außen den zuletzt angezeigten bzw. neuen
        // Ziffern-Schritt ablesen zu können.
        let normalized_last = pos_last.rem_euclid(max_val);
        let normalized_new = new_pos.rem_euclid(max_val);

        self.step_last = normalized_last.floor() as u8;
        self.new_step = normalized_new.floor() as u8;
    }
}
