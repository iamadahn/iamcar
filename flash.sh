openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c "build/f411_car.elf verify reset exit"
