#[derive(Debug, Clone)]
pub enum InternalEvents {
	IMU(IMU),
	BatterySwitch(bool),
	BatteryVoltage(f32),
}

#[derive(Debug, Clone)]
pub struct IMU {
	key: String,
	value: Option<String>,
}