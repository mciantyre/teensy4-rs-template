@echo off

SET scriptbuildflag=--release
SET scriptbin={{crate_name}}
IF "%1"=="--debug" (SET "scriptbuildflag=")
IF NOT DEFINED scriptbuildflag (
	SET scriptbuildmode=debug
) ELSE (
	SET scriptbuildmode=release
)
cargo build %scriptbuildflag% || goto :error
if exist out rmdir /S /Q out || goto :error
mkdir out || goto :error
copy "target\thumbv7em-none-eabihf\%scriptbuildmode%\%scriptbin%" "out\%scriptbin%" || goto :error
REM arm-none-eabi-objdump -d -S -C "out\%scriptbin%" > "out\%scriptbin%.lst" || goto :error
REM arm-none-eabi-objdump -t -C "out\%scriptbin%" > "out\%scriptbin%.sym" || goto :error
arm-none-eabi-objcopy -O ihex -R .eeprom "out\%scriptbin%" "out\%scriptbin%.hex" || goto :error
echo Now use Teensy Loader to load out\%scriptbin%.hex onto the Teensy 4.
goto :EOF

:error
echo Failed with error code %errorlevel%.
exit /b %errorlevel%
