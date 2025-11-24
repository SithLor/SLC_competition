#include <Servo.h>

Servo sparkMax;

void setup() {
    sparkMax.attach(9); // Connect SPARK MAX PWM wire to pin 9
}

void loop() {
    // Neutral (no movement)
    sparkMax.writeMicroseconds(1500);
    delay(2000);

    //blink led 
    for (int i = 0; i < 5; i++) {
        digitalWrite(LED_BUILTIN, HIGH); // Turn the LED on
        delay(500);                      // Wait for half a second
        digitalWrite(LED_BUILTIN, LOW);  // Turn the LED off
        delay(500);                      // Wait for half a second
    }
    // Full reverse
    sparkMax.writeMicroseconds(1000);
    delay(2000);

    // blink led 
    // Full forward
    sparkMax.writeMicroseconds(2000);
    delay(2000);
}
