# Weather Widget

![1701786787_grim](https://github.com/AndreiLubinets/weather-widget/assets/93917217/8ad33df5-ab1d-42df-b675-ffebf898bbf5)

Weather widget for Hyprland desktop powered by https://www.weatherapi.com.

**TODO**

### Build:
Install gtk3
```bash
export API_KEY=<Insert api key>
cargo build --release
```



### Configuration File Example:

Configuration is located at `$HOME/.config/weather-widget/Config.toml`

```toml
uri = "http://api.weatherapi.com/v1/"
location = "Paris, France"
bg_color = "#2a2a3e" #Optional

[size] #Optional
width = 500.0
height = 400.0
```
