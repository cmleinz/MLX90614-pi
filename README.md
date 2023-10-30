# MLX90614-pi

A minimal web service for querying data from the MLX90614 infrared thermometer.
This is a "batteries included" implemtation including a small REST API, 
serving the data from the sensor in JSON form. Intended for use on the 
raspberry pi.

## Installation

```
git clone https://github.com/cmleinz/MLX90614-pi.git
cd MLX90614-pi
cargo build --release
sudo cp target/release/mlx90614-pi /usr/bin/
sudo cp mlx90614.service /etc/systemd/system/
sudo systemd start mlx90614
sudo systemd enable mlx90614
```
