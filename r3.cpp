//marco beacuse fuckyou
#define MIN_VOLTAGE 3.30f
#define MAX_VOLTAGE 4.20f
#define VOLT_TO_PERCENT(v) (int)( ((v) - MIN_VOLTAGE) * (100.0f / (MAX_VOLTAGE - MIN_VOLTAGE)))

// Pin assignments (use your board's actual wiring)
const int BLUE_PIN  = 5;
const int GREEN_PIN = 6;
const int RED_PIN   = 7;

// If you keep this helper, make sure to configure pins as OUTPUT,
// then set an initial level with digitalWrite.
void setup_led_rgb_light() {
  int pin[] = { BLUE_PIN, GREEN_PIN, RED_PIN };
  int pin_len = sizeof(pin) / sizeof(pin[0]);

  for (int i = 0; i < pin_len; i++) {
    pinMode(pin[i], OUTPUT);     // set direction
    digitalWrite(pin[i], LOW);   // initial OFF for common-cathode
  }
}

class RGB {
 public:
  int red_pin;
  int green_pin;
  int blue_pin;

  // Track which color is currently on: 0=off, 1=red, 2=green, 3=blue
  int color_state;

  // Set commonAnode=true if your LED is common anode (active LOW)
  bool commonAnode;

  RGB(int r_p, int g_p, int b_p, bool commonAnode = false)
    : red_pin(r_p),
      green_pin(g_p),
      blue_pin(b_p),
      color_state(0),
      commonAnode(commonAnode) {}

  void begin() {
    pinMode(red_pin, OUTPUT);
    pinMode(green_pin, OUTPUT);
    pinMode(blue_pin, OUTPUT);
    off();
  }

  // Only one color at a time
  void red()   { setColor(1); }
  void green() { setColor(2); }
  void blue()  { setColor(3); }
  void off()   { setColor(0); }

  void setColor(int c) {
    // normalize
    if (c < 0 || c > 3) c = 0;

    // turn all off first
    writePin(red_pin,   false);
    writePin(green_pin, false);
    writePin(blue_pin,  false);

    // then enable the requested single color
    switch (c) {
      case 1: writePin(red_pin,   true); break;
      case 2: writePin(green_pin, true); break;
      case 3: writePin(blue_pin,  true); break;
      default: break; // off
    }
    color_state = c;
  }

 private:
  void writePin(int pin, bool on) {
    // For common anode: ON=LOW, OFF=HIGH
    // For common cathode: ON=HIGH, OFF=LOW
    if (commonAnode) {
      digitalWrite(pin, on ? LOW : HIGH);
    } else {
      digitalWrite(pin, on ? HIGH : LOW);
    }
  }
};
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
RGB led(RED_PIN, GREEN_PIN, BLUE_PIN, /*commonAnode=*/true);


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
