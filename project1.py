import RPi.GPIO as GPIO
import time
import threading
import os

def read_temperature():
    f = open("/sys/bus/w1/devices/28-00000755f9f2/w1_slave", 'r')
    lines = f.readlines()
    f.close()
    return lines

def parse_temperature():
    lines = read_temperature()
    while lines[0].strip()[-3:] != 'YES':
        time.sleep(0.2)
        lines = read_temperature()

    temp_output = lines[1].find('t=')

    if temp_output != -1:
        temp_string = lines[1].strip()[temp_output+2:]
        temp_c = float(temp_string) / 1000.0
        temp_f = temp_c * 9.0 / 5.0 + 32.0
        return temp_f

def setup():
    os.system('modprobe w1-gpio')
    os.system('modprobe w1-therm')

    GPIO.setmode(GPIO.BCM)
    GPIO.setup(2, GPIO.IN, pull_up_down=GPIO.PUD_DOWN)
    GPIO.setup(3, GPIO.IN, pull_up_down=GPIO.PUD_DOWN)
    GPIO.add_event_detect(2, GPIO.FALLING, callback=button2, bouncetime=300)
    GPIO.add_event_detect(3, GPIO.FALLING, callback=button3, bouncetime=300)
    GPIO.setup(18, GPIO.OUT)
    GPIO.setup(5, GPIO.OUT)
    GPIO.setup(24, GPIO.OUT)

    print("Press Ctrl-C to exit")

def button3(channel):
    print("button 3 pressed")
    global button_action
    if button_action:
        button_action = False
    else:
        button_action = True

def button2(channel):
    print("button 2 pressed")
    global rate
    global value
    global button_action

    if button_action:
        if rate.is_paused():
            print("unlocking blinker rate")
            rate.start()
        else:
            print("locking blinker rate")
            rate.pause()
    else:
        if value.is_paused():
            print("unlocking dimmer rate")
            value.start()
        else:
            print("locking dimmer rate")
            value.pause()

def brightness_thread(run_event, pwm):
    while run_event.is_set():
        global value
        pwm.ChangeDutyCycle(value.next())
        time.sleep(0.5)

def temperature_thread(run_event, pwm24):
    while run_event.is_set():
        temp = parse_temperature()
        print("Temperature in F: {}".format(temp))
        if temp <= 72.9:
            GPIO.output(24, GPIO.LOW)
        else:
            if temp > 74:
                pwm24.ChangeDutyCycle(temp)

        time.sleep(5)

class ValueSource:
    def __init__(self, values):
        self.paused = False
        self.values = values
        self.index = 0

    def start(self):
        self.paused = False

    def pause(self):
        self.paused = True

    def is_paused(self):
        return self.paused

    def next(self):
        old_index = self.index

        if self.paused:
            return self.values[old_index]
        
        if self.index == len(self.values) - 1:
            self.index = 0
        else:
            self.index = old_index + 1

        return self.values[old_index]
        
button_action = True
rate = ValueSource([x for x in range(6, 0, -1)])
value = ValueSource([x for x in range(0, 100, 5)])

def main():
    setup()
    pwm18 = GPIO.PWM(18, 100)
    pwm24 = GPIO.PWM(24, 100)
    pwm18.start(0)
    pwm24.start(0)

    run_event = threading.Event()
    run_event.set()
    brightness_led = threading.Thread(target = brightness_thread, args = (run_event, pwm18))
    temperature_led = threading.Thread(target = temperature_thread, args = (run_event, pwm24))

    brightness_led.start()
    time.sleep(0.1)
    temperature_led.start()

    try:
        while True:
            global rate
            sleep_time = rate.next()
            GPIO.output(5, GPIO.HIGH)
            time.sleep(sleep_time)
            GPIO.output(5, GPIO.LOW)
            time.sleep(sleep_time)

    except KeyboardInterrupt:
        print("Exiting, please wait.")
        run_event.clear()
        brightness_led.join()
        temperature_led.join()
        pwm18.stop()
        pwm24.stop()
        GPIO.remove_event_detect(2)
        GPIO.remove_event_detect(3)
        GPIO.cleanup()
        print("all threads successfully completed")

if __name__ == '__main__':
    main()
