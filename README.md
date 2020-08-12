# MeteoNook Alpha

A weather prediction tool for Animal Crossing: New Horizons, created by [@_Ninji](https://twitter.com/_Ninji). This is a full rewrite of the original proof-of-concept tool, now using Vue.js with BootstrapVue and with a core written in Rust.

The official release is available at: https://wuffs.org/acnh/weatherTest/

Documentation and code here (except where specified) is (c) 2020 Ash Wolf ("Ninji"), and available under the GNU Affero General Public License 3.0 as provided in the LICENSE file.

Special thanks to the following contributors:

- **Testing and suggestions for v1**: Ahri, Alexis, andrewtabs, astro, capstone, ctar17, Denna, GoldenCrater, rainbowannie, robotic_scarab
- **Spanish translation proofreading**: [@jerdieh](https://twitter.com/jerdieh)
- **Italian translation**: Thordgar
- **French translation**: [@Swpolo](https://github.com/Swpolo)
- **German translation**: [@TheVaan](https://github.com/TheVaan)
- **German (CH) translation**: [@einfallstoll](https://github.com/einfallstoll)
- **Chinese (Simplified) translation**: [@IceLitty](https://github.com/IceLitty)
- **Chinese (Traditional) translation**: [@Small-Ku](https://github.com/Small-Ku)
- **Taiwanese translation**: Laurence Chen
- **Japanese translation**: [@yuma-m](https://github.com/yuma-m), [@Starwort](https://github.com/Starwort)
- **Russian translation**: [@yozhique_acnh](https://twitter.com/yozhique_acnh)
- **Korean translation**: Like_it, Kuiprux
- **Dutch translation**: Angela, Mees Muller, Bjørn, Thijsvb
- **Czech translation**: [@lukix1995](https://twitter.com/lukix1995)

Translations are managed centrally using POEditor. Join the project here to contribute: https://poeditor.com/join/project/CerldM8Yc9 - There's no good way for me to map POEditor accounts to users for crediting above and in the app, so to be added, please let me know who you are by submitting a pull request, commenting on issue #6 or mentioning me on Twitter.

Additionally, there's no obligation to do so, but if you'd like to support my work on this project and other AC:NH research: [PayPal.me](https://paypal.me/trashcurl) | [Ko-fi](https://ko-fi.com/ninji_) | [Monzo](https://monzo.me/ninji) (UK)

## Roadmap/to-do list

- Refresh overview when the day changes at 5am
- Forecast exports
- Some sort of hosted API
- Dark mode
- Toggle to use system-agnostic icons for those stuck on Windows 7 without emoji support
