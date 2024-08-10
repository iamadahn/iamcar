openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c "program build/f411_car.elf verify reset exit"
