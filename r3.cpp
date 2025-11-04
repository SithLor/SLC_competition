//marco beacuse fuckyou
#define MIN_VOLTAGE 3.30f
#define MAX_VOLTAGE 4.20f
#define VOLT_TO_PERCENT(v) (int)( ((v) - MIN_VOLTAGE) * (100.0f / (MAX_VOLTAGE - MIN_VOLTAGE)))


class Battery {
  public:
    float battery_voltage;
    int battery_percentage;
    int battery_pin;
    Battery() {
      battery_voltage = 0;
      battery_percentage = 0;
      battery_pin = A0;
    }
    Battery(int pin) {
      battery_voltage = 0;
      battery_percentage = 0;
      battery_pin = pin;
    }

    int get_battery_percent();
    float get_battery_voltage();
    void refresh_data();
};
int Battery::get_battery_percent() {
  float v = battery_voltage;

  if (v >= 4.20) return 100;
  if (v <= 3.30) return 0;

  // Map voltage range 3.30–4.20 to 0–100%
  // 3.30V = 0%, 4.20V = 100%
  // example
  // int percent = (int)(((v - MIN_VOLTAGE) * (100.0f / (MAX_VOLTAGE - MIN_VOLTAGE))));
  int percent = VOLT_TO_PERCENT(v);

  // clamp
  if (percent < 0) percent = 0;
  if (percent > 100) percent = 100;

  return percent;
}

float Battery::get_battery_voltage() {
  battery_voltage = (analogRead(battery_pin) / 1023.0) * 5.0;
  return battery_voltage;
}
void Battery::refresh_data() {
  get_battery_voltage();
  battery_percentage = get_battery_percent();
}

Battery bat(A0);



void setup() {
  Serial.begin(9600);
  pinMode(LED_BUILTIN, OUTPUT);
}
void loop() {
  bat.refresh_data();
  Serial.print("Voltage: ");
  Serial.print(bat.battery_voltage);
  Serial.print("V  |  Percent: ");
  Serial.println(bat.battery_percentage);

  if (bat.battery_voltage < 3.0) digitalWrite(LED_BUILTIN, HIGH);
  else digitalWrite(LED_BUILTIN, LOW);

  delay(500);
}
