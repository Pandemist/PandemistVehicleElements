use lotus_script::delta;

use crate::elements::{
    std::{helper::bool_to_int_pos_neg, piecewise_linear_function::PiecewiseLinearFunction},
    tech::blinker::Blinkrelais,
};

const HUBLIFT_HIGHT_SO: f32 = 1.0;
const HUBLIFT_HIGHT_BAHNSTEIG: f32 = 0.3;
const ARRETIERUNG_CHANGE_TIME: f32 = 2.0;
const HUBLIFT_RAMPE_SPEED: f32 = 1.0 / 6.0;
const HUBLIFT_ABROLLSCHUTZ_SPEED: f32 = 1.0 / 1.5;
const HUBLIFT_HIGHT_SPEED: f32 = 1.0 / 3.0;
const HUBLIFT_DOORWARN_INTERVAL: f32 = 1.0;
const HUBLIFT_DOORWARN_INTERVAL_HALF: f32 = HUBLIFT_DOORWARN_INTERVAL / 2.0;

#[derive(Debug)]
pub struct Hublift {
    height_pos: f32,
    rampe_pos: f32,
    abrollschutz_pos: f32,
    rampenklappe_pos: f32,
    rampenubergang_pos: f32,

    paustimer: f32,

    arretierung: f32,

    _defekt: bool,

    _sicherung_steuerung: bool,
    _sicherung_leistung: bool,
    _sicherung_steuersig: bool,

    target_level: i8,
    aktion_fertig: bool,

    door_open_lock: bool,

    vorbereitet: bool,

    warnrelais: Blinkrelais,

    snd_warn: i8,
    snd_a_down: i8,
    snd_a_up: i8,
    snd_a_up_end: i8,
    snd_b_down: i8,
    snd_b_up: i8,
    snd_b_up_end: i8,
    snd_barrier_down: i8,
    snd_barrier_up: i8,

    klappe_func: PiecewiseLinearFunction,
    uebergang_func: PiecewiseLinearFunction,

    closed: bool,
}

impl Hublift {
    pub fn new() -> Self {
        let mut hl = Hublift {
            closed: true,
            warnrelais: Blinkrelais::new(
                HUBLIFT_DOORWARN_INTERVAL,
                HUBLIFT_DOORWARN_INTERVAL_HALF,
                0.1,
            ),
            height_pos: 0.0,
            rampe_pos: 0.0,
            abrollschutz_pos: 0.0,
            rampenklappe_pos: 0.0,
            rampenubergang_pos: 0.0,
            paustimer: 0.0,
            arretierung: 0.0,
            _defekt: false,
            _sicherung_steuerung: false,
            _sicherung_leistung: false,
            _sicherung_steuersig: false,
            target_level: 0,
            aktion_fertig: false,
            door_open_lock: false,
            vorbereitet: false,
            snd_warn: 0,
            snd_a_down: 0,
            snd_a_up: 0,
            snd_a_up_end: 0,
            snd_b_down: 0,
            snd_b_up: 0,
            snd_b_up_end: 0,
            snd_barrier_down: 0,
            snd_barrier_up: 0,
            klappe_func: PiecewiseLinearFunction::new(),
            uebergang_func: PiecewiseLinearFunction::new(),
        };

        hl.klappe_func.add_pair(0.0, 0.0);
        hl.klappe_func.add_pair(0.038, 0.0);
        hl.klappe_func.add_pair(0.060, 0.090);
        hl.klappe_func.add_pair(0.090, 0.190);
        hl.klappe_func.add_pair(0.122, 0.316);
        hl.klappe_func.add_pair(0.156, 0.424);
        hl.klappe_func.add_pair(0.182, 0.468);
        hl.klappe_func.add_pair(0.208, 0.512);
        hl.klappe_func.add_pair(0.252, 0.584);
        hl.klappe_func.add_pair(0.296, 0.652);
        hl.klappe_func.add_pair(0.320, 0.688);
        hl.klappe_func.add_pair(0.364, 0.752);
        hl.klappe_func.add_pair(0.388, 0.786);
        hl.klappe_func.add_pair(0.412, 0.822);
        hl.klappe_func.add_pair(0.430, 0.844);
        hl.klappe_func.add_pair(0.448, 0.854);
        hl.klappe_func.add_pair(1.0, 0.854);

        hl.uebergang_func.add_pair(0.0, 0.0);
        hl.uebergang_func.add_pair(0.02, 0.1);
        hl.uebergang_func.add_pair(0.038, 0.2);
        hl.uebergang_func.add_pair(0.058, 0.3);
        hl.uebergang_func.add_pair(0.078, 0.4);
        hl.uebergang_func.add_pair(0.098, 0.5);
        hl.uebergang_func.add_pair(0.120, 0.6);
        hl.uebergang_func.add_pair(0.138, 0.7);
        hl.uebergang_func.add_pair(0.156, 0.8);
        hl.uebergang_func.add_pair(0.174, 0.9);
        hl.uebergang_func.add_pair(0.194, 1.0);
        hl.uebergang_func.add_pair(0.212, 1.116);
        hl.uebergang_func.add_pair(0.3, 1.605);

        hl
    }

    pub fn tick(&mut self, mut target: i8, _notablegen: bool, _spannung: f32) {
        if self.closed {
            self.warnrelais.tick();
        } else {
            self.warnrelais.reset();
        }

        let height_pos_last = self.height_pos;
        let rampe_pos_last = self.rampe_pos;
        let abrollschutz_pos_last = self.abrollschutz_pos;
        let arretierung_last = self.arretierung;

        if self.door_open_lock {
            if target == 0 {
                self.aktion_fertig = false;
            }

            if self.aktion_fertig {
                target = 0;
            }

            self.paustimer = (self.paustimer * delta()).max(0.0);

            // Paustimer <= 0
            if self.paustimer <= 0.0 {
                // Absenken
                if target < 0 && self.target_level > 1 {
                    self.arretierung = (self.arretierung * delta()).min(ARRETIERUNG_CHANGE_TIME);

                    // Auf Bahnsteigniveau absenken
                    if self.target_level == 2 {
                        // Nur arbeiten, wenn Arretierung ist gelöst
                        if self.arretierung >= ARRETIERUNG_CHANGE_TIME {
                            // Vorbereitung abgeschlossen
                            if arretierung_last != ARRETIERUNG_CHANGE_TIME
                                && self.arretierung == ARRETIERUNG_CHANGE_TIME
                            {
                                self.aktion_fertig = true;
                            }

                            // Absenken
                            self.height_pos = (self.height_pos + HUBLIFT_HIGHT_SPEED * delta())
                                .min(HUBLIFT_HIGHT_BAHNSTEIG);

                            if self.height_pos == HUBLIFT_HIGHT_BAHNSTEIG {
                                self.aktion_fertig = true;
                            }
                        }
                    }
                    // Auf Straßenneiveau absenken
                    else if self.target_level == 3 {
                        if !self.vorbereitet {
                            // Rampe ausfahren
                            self.rampe_pos =
                                (self.rampe_pos + HUBLIFT_RAMPE_SPEED * delta()).min(1.0);

                            // Abrollschutz aufstellen
                            if self.rampe_pos == 1.0 {
                                self.abrollschutz_pos = (self.abrollschutz_pos
                                    + HUBLIFT_ABROLLSCHUTZ_SPEED * delta())
                                .min(1.0);
                            }

                            // Fertigmeldung
                            if self.rampe_pos == 1.0
                                && self.abrollschutz_pos == 1.0
                                && self.arretierung >= ARRETIERUNG_CHANGE_TIME
                            {
                                self.vorbereitet = true;
                                self.aktion_fertig = true;
                            }
                        } else {
                            // Rampe absenken
                            self.height_pos = (self.height_pos + HUBLIFT_HIGHT_SPEED * delta())
                                .min(HUBLIFT_HIGHT_SO);

                            // Abrollschutz absenken
                            if self.height_pos >= HUBLIFT_HIGHT_SO {
                                self.abrollschutz_pos = (self.abrollschutz_pos
                                    - HUBLIFT_ABROLLSCHUTZ_SPEED * delta())
                                .max(0.0);
                            }

                            // Fertigmeldung
                            if self.height_pos >= HUBLIFT_HIGHT_SO && self.abrollschutz_pos == 0.0 {
                                self.aktion_fertig = true;
                            }
                        }
                    }
                }
                // Heben
                else if target > 0 {
                    // Einfahren, wenn oben
                    if self.height_pos <= 0.0 {
                        self.vorbereitet = false;

                        // Abrollschutz wieder senken
                        self.abrollschutz_pos =
                            (self.abrollschutz_pos - HUBLIFT_ABROLLSCHUTZ_SPEED * delta()).max(0.0);

                        // Rampe einfahren
                        if self.abrollschutz_pos == 0.0 {
                            self.rampe_pos =
                                (self.rampe_pos - HUBLIFT_RAMPE_SPEED * delta()).max(0.0);
                        }

                        // Kurze Pause, wenn die Rampe Endlage erreicht hat
                        if self.rampe_pos == 0.0 && rampe_pos_last != 0.0 {
                            self.paustimer = 0.5;
                        }

                        // Arretierung wieder feststellen
                        if self.height_pos <= 0.0 && self.rampenklappe_pos == 0.0 {
                            self.arretierung = (self.arretierung - delta()).max(0.0);
                        }

                        // Fertigmeldung
                        if self.arretierung <= 0.0 {
                            self.aktion_fertig = true;
                        }
                    }
                    // Hochfahren
                    else {
                        // Bei Straßenniveau
                        if self.abrollschutz_pos > 0.0 {
                            // Abrollschutz Heben
                            self.abrollschutz_pos = (self.abrollschutz_pos
                                + HUBLIFT_ABROLLSCHUTZ_SPEED * delta())
                            .min(1.0);

                            // Wieder Anheben, wenn Abrollschutz aufgestellt
                            if self.abrollschutz_pos == 1.0 {
                                self.height_pos =
                                    (self.height_pos - HUBLIFT_HIGHT_SPEED * delta()).max(0.0);
                            }
                        }
                        // Wieder Anheben
                        else {
                            self.height_pos =
                                (self.height_pos - HUBLIFT_HIGHT_SPEED * delta()).max(0.0);
                        }

                        // Fertigmeldung
                        if self.height_pos == 0.0 {
                            self.aktion_fertig = true;
                        }
                    }
                }
            }
        }

        if target != 0 && self.target_level > 0 && !self.aktion_fertig {
            self.snd_warn = 1;
        } else {
            self.snd_warn = -1;
        }

        self.snd_a_down = bool_to_int_pos_neg(
            (self.height_pos < height_pos_last) || (self.arretierung < arretierung_last),
        );
        self.snd_a_up = bool_to_int_pos_neg(
            (self.height_pos > height_pos_last) || (self.arretierung > arretierung_last),
        );
        self.snd_a_up_end = bool_to_int_pos_neg(self.height_pos == 1.0 && height_pos_last != 1.0);
        self.snd_b_down = bool_to_int_pos_neg(self.rampe_pos < rampe_pos_last);
        self.snd_b_up = bool_to_int_pos_neg(self.rampe_pos > rampe_pos_last);
        self.snd_b_up_end = bool_to_int_pos_neg(self.rampe_pos == 1.0 && rampe_pos_last != 1.0);
        self.snd_barrier_down =
            bool_to_int_pos_neg(self.abrollschutz_pos != 1.0 && abrollschutz_pos_last == 1.0);
        self.snd_barrier_up =
            bool_to_int_pos_neg(self.abrollschutz_pos != 0.0 && abrollschutz_pos_last == 0.0);

        self.rampenklappe_pos = self.klappe_func.get_value(self.rampe_pos);
        self.rampenubergang_pos = self.uebergang_func.get_value(self.height_pos);

        self.closed = self.arretierung == 0.0;
    }
}
