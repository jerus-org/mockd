# mockd

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Rust][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/mockd.svg
[crates-url]: https://crates.io/crates/mockd
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/mockd/blob/main/LICENSE
[actions-badge]: https://github.com/jerusdp/mockd/actions/workflows/general.yml/badge.svg?branch=main
[actions-url]: https://github.com/jerusdp/mockd/actions/workflows/general.yml
[version-badge]: https://img.shields.io/badge/rust-1.56.0+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/mockd/badge.svg
[docs-url]:  https://docs.rs/mockd
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

Update to [fakeit](https://github.com/PumpkinSeed/fakeit), a Rust port of the famous [Go fakeit](https://github.com/brianvoe/gofakeit) library with more than 130 functions.

## Usage

- [Crates.io/fakeit](https://crates.io/crates/mockd)
- [docs.rs](https://docs.rs/mockd)

## Functions
<!-- markdownlint-disable MD007 -->
- [mockd](#mockd)
  - [Usage](#usage)
  - [Functions](#functions)
    - [address (16 functions)](#address-16-functions)
      - [animal (6 functions)](#animal-6-functions)
        - [beer (8 functions)](#beer-8-functions)
        - [bool (1 functions)](#bool-1-functions)
        - [color (4 functions)](#color-4-functions)
        - [company (4 functions)](#company-4-functions)
        - [contact (4 functions)](#contact-4-functions)
        - [currency (4 functions)](#currency-4-functions)
        - [datetime (14 functions)](#datetime-14-functions)
        - [file (2 functions)](#file-2-functions)
        - [generator (1 function)](#generator-1-function)
        - [hacker (6 functions)](#hacker-6-functions)
        - [hipster (3 functions)](#hipster-3-functions)
        - [image (1 function)](#image-1-function)
        - [internet (7 functions)](#internet-7-functions)
        - [job (4 functions)](#job-4-functions)
        - [language (3 functions)](#language-3-functions)
        - [log-level (3 functions)](#log-level-3-functions)
        - [name (5 functions)](#name-5-functions)
        - [password (1 function)](#password-1-function)
        - [payment (6 functions)](#payment-6-functions)
        - [person (3 functions)](#person-3-functions)
        - [status code (2 functions)](#status-code-2-functions)
        - [unique (2 functions)](#unique-2-functions)
        - [user agent (8 functions)](#user-agent-8-functions)
        - [vehicle (6 functions)](#vehicle-6-functions)
        - [words (6 functions)](#words-6-functions)
<!-- markdownlint-enable MD007 -->
### address (16 functions)

```rust
use mockd::address;

fn main() {
    let data = address::info(); // address::Info struct
    let data = address::street(); // street: 1128 South North Dakota borough
    let data = address::street_number(); // street_number: 3155
    let data = address::street_prefix(); // street_prefix: Port
    let data = address::street_name(); // street_name: Kansas
    let data = address::street_suffix(); // street_suffix: mouth
    let data = address::city(); // city: Schmelerburgh
    let data = address::state(); // state: Kentucky
    let data = address::state_abr(); // state_abr: WA
    let data = address::zip(); // zip: 75221
    let data = address::country(); // country: Romania
    let data = address::country_abr(); // country_abr: BI
    let data = address::latitude(); // latitude: -69.14192
    let data = address::latitude_in_range(-30 as f64, 30 as f64); // latitude_in_range: -18.35571
    let data = address::longitude(); // longitude: 113.12952
    let data = address::longitude_in_range(-30 as f64, 30 as f64); // longitude_in_range: -16.484156
}
```

#### animal (6 functions)

```rust
use mockd::animal;

fn main() {
    let data = animal::pet_name(); // pet_name: Squeakers
    let data = animal::animal(); // animal: salmon
    let data = animal::type_of(); // type_of: fish
    let data = animal::farm(); // farm: Sheep
    let data = animal::cat(); // cat: Oriental Shorthair
    let data = animal::dog(); // dog: Rottweiler
}
```

##### beer (8 functions)

```rust
use mockd::beer;

fn main() {
    let data = beer::name(); // name: Sierra Nevada Bigfoot Barleywine Style Ale
    let data = beer::style(); // style: Porter
    let data = beer::hop(); // hop: Equinox
    let data = beer::yeast(); // yeast: 1084 - Irish Ale
    let data = beer::malt(); // malt: Roasted barley
    let data = beer::ibu(); // ibu: 75 IBU
    let data = beer::alcohol(); // alcohol: 2.943696 %
    let data = beer::blg(); // blg: 7.4607124°Blg
}
```

##### bool (1 functions)

```rust
use mockd::bool;

fn main() {
    let data = bool::bool(); // true / false
}
```

##### color (4 functions)

```rust
use mockd::color;

fn main() {
    let data = color::full(); // full: LightYellow
    let data = color::hex(); // hex: #662461
    let data = color::safe(); // safe: black
    let data = color::rgb(); // rgb: [162, 98, 22]
}
```

##### company (4 functions)

```rust
use mockd::company;

fn main() {
    let data = company::company(); // company: Rowe-Schoen
    let data = company::company_suffix(); // company_suffix: Inc
    let data = company::buzzword(); // buzzword: systemic
    let data = company::bs(); // bs: strategic
}
```

##### contact (4 functions)

```rust
use mockd::contact;

fn main() {
    let data = contact::info(); // contact::Info
    let data = contact::phone(); // phone: 5173757868
    let data = contact::phone_formatted(); // phone_formatted: 382.450.6544
    let data = contact::email(); // email: benkuvalis@marks.org
}
```

##### currency (4 functions)

```rust
use mockd::currency;

fn main() {
    let data = currency::compact(); // currency::Info
    let data = currency::short(); // short: SRD
    let data = currency::long(); // long: Burundi Franc
    let data = currency::price(1 as f64, 123 as f64); // price: 53.7
}
```

##### datetime (14 functions)

```rust
use mockd::datetime;

fn main() {
    let data = datetime::month(); // month: 10
    let data = datetime::day(); // day: 10
    let data = datetime::week_day(); // week_day: 6
    let data = datetime::year(); // year: 1986
    let data = datetime::hour(); // hour: 10
    let data = datetime::minute(); // minute: 10
    let data = datetime::second(); // second: 10
    let data = datetime::nanosecond(); // nanosecond: 959678991
    let data = datetime::timezone(); // timezone: SA Pacific Standard Time
    let data = datetime::timezone_full(); // timezone_full: (UTC-04:00) Atlantic Time (Canada)
    let data = datetime::timezone_abv(); // timezone_abv: BST
    let data = datetime::timezone_offset(); // timezone_offset: 13
    let data = datetime::date_range("RFC3339", "RFC3339"); // date_range: 1979-01-06 23:03:10.918301212 UTC
    let data = datetime::date(); // date: 1979-01-06 23:03:10.918301212 UTC
}
```

##### file (2 functions)

```rust
use mockd::file;

fn main() {
    let data = file::mime_type(); // mime_type: text/x-fortran
    let data = file::extension(); // extension: aspx
}
```

##### generator (1 function)

```rust
use mockd::generator;

fn main() {
    let data = generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string()); // data: Watson Connelly baileeprosacco@smitham.biz 6d0e0a
    // More details about this later
}
```

##### hacker (6 functions)

```rust
use mockd::hacker;

fn main() {
    let data = hacker::phrase(); // phrase: parsing the sensor won't do anything, we need to bypass the open-source AGP sensor!
    let data = hacker::abbreviation(); // abbreviation: PCI
    let data = hacker::adjective(); // adjective: bluetooth
    let data = hacker::noun(); // noun: protocol
    let data = hacker::verb(); // verb: copy
    let data = hacker::ingverb(); // ingverb: transmitting
}
```

##### hipster (3 functions)

```rust
use mockd::hipster;

fn main() {
    let data = hipster::word(); // word: fingerstache
    let data = hipster::sentence(12); // sentence: Itaque aliquid id ex repudiandae adipisci quibusdam excepturi deleniti qui alias animi.
    let data = hipster::paragraph(3, 4, 40, " ".to_string()); // paragraph: Voluptas minima delectus voluptatibus earum rerum accusamus consequatur sunt....
}
```

##### image (1 function)

```rust
use mockd::image;

fn main() {
    let data = image::url(500, 500); // url: https://picsum.photos/500/500
}
```

##### internet (7 functions)

```rust
use mockd::internet;

fn main() {
    let data = internet::domain_name(); // domain_name: productvisualize.net
    let data = internet::http_method(); // http_method: DELETE
    let data = internet::domain_suffix(); // domain_suffix: biz
    let data = internet::ipv4_address(); // ipv4_address: 196.140.182.201
    let data = internet::ipv6_address(); // ipv6_address: 2001:cafe:1248:1dc7:17dd:19f4:8798:621d
    let data = internet::mac_address(); // mac_address: 2D:3F:7E:5D:61:C1
    let data = internet::username(); // username: Nienow1881
}
```

##### job (4 functions)

```rust
use mockd::job;

fn main() {
    let data = job::info(); // job::Info
    let data = job::title(); // title: Executive
    let data = job::descriptor(); // descriptor: International
    let data = job::level(); // level: Solutions
}
```

##### language (3 functions)

```rust
use mockd::language;

fn main() {
    let data = language::random(); // random: Tatar
    let data = language::abbreviation(); // abbreviation: co
    let data = language::programming(); // programming: Rust
}
```

##### log-level (3 functions)

```rust
use mockd::log_level;

fn main() {
    let data = log_level::general(); // general: info
    let data = log_level::syslog(); // syslog: crit
    let data = log_level::apache(); // apache: debug
}
```

##### name (5 functions)

```rust
use mockd::name;

fn main() {
    let data = name::full(); // full: Keyshawn Auer
    let data = name::first(); // first: Brycen
    let data = name::last(); // last: Hartmann
    let data = name::prefix(); // prefix: Mr.
    let data = name::suffix(); // suffix: PhD
}
```

##### password (1 function)

```rust
use mockd::password;

fn main() {
    let data = password::generate(upper, numeric, special, num); // #9e1Vv5s&Ng8L-#9@=!6+s1+0@R
}
```

##### payment (6 functions)

```rust
use mockd::payment;

fn main() {
    let data = payment::credit_card(); // payment::CreditCard
    let data = payment::credit_card_type(); // credit_card_type: Discover
    let data = payment::credit_card_number(); // credit_card_number: 341545247171534
    let data = payment::credit_card_luhn_number(); // @TODO
    let data = payment::credit_card_exp(); // credit_card_exp: 04/21
    let data = payment::credit_card_cvv(); // credit_card_cvv: 537
}
```

##### person (3 functions)

```rust
use mockd::person;

fn main() {
    let data = person::info(); // person::Info
    let data = person::ssn(); // ssn: 792671651
    let data = person::gender(); // gender: male
}
```

##### status code (2 functions)

```rust
use mockd::status_code;

fn main() {
    let data = status_code::simple(); // simple: 404
    let data = status_code::general(); // general: 400
}
```

##### unique (2 functions)

```rust
use mockd::unique;

fn main() {
    let data = unique::uuid_v1(); // uuid_v1: 13be40a6-1dd2-11b2-802a-010203040506
    let data = unique::uuid_v4(); // uuid_v4: a474961e-936a-4897-966a-15fcff7bbc87
}
```

##### user agent (8 functions)

```rust
use mockd::unique;

fn main() {
    let data = user_agent::chrome(); // chrome: Mozilla/5.0 (X11; Linux i686) AppleWebKit/532 (KHTML, like Gecko) Chrome/36.0.861.0 Mobile Safari/532
    let data = user_agent::firefox(); // firefox: Mozilla/5.0 (X11; Linux x86_64; rv:7.0) Gecko/2005-5-27 Firefox/36.0
    let data = user_agent::safari(); // safari: Mozilla/5.0 (Windows; U; Windows NT 6.2) AppleWebKit/531.23.3 (KHTML, like Gecko) Version/4.0 Safari/531.23.3
    let data = user_agent::opera(); // opera: Opera/8.22 (Macintosh; PPC Mac OS X 10_6_8; en-US) Presto/2.11.181 Version/12.00
    let data = user_agent::linux_platform_token(); // linux_platform_token: X11; Linux x86_64
    let data = user_agent::mac_platform_token(); // mac_platform_token: Macintosh; U; PPC Mac OS X 10_6_2
    let data = user_agent::windows_platform_token(); // windows_platform_token: Windows 98; Win 9x 4.90
    let data = user_agent::random_platform(); // random_platform: Macintosh; Intel Mac OS X 10_7_5
}
```

##### vehicle (6 functions)

```rust
use mockd::vehicle;

fn main() {
    let data = vehicle::info(); // vehicle::Info
    let data = vehicle::vehicle_type(); // vehicle_type: Passenger car mini
    let data = vehicle::fuel(); // fuel: Electric
    let data = vehicle::transmission_gear(); // transmission_gear: Automatic
    let data = vehicle::car_maker(); // car_maker: Chevrolet
    let data = vehicle::car_model(); // car_model: Gti
}
```

##### words (6 functions)

```rust
use mockd::words;

fn main() {
    let data = words::word(); // word: saepe
    let data = words::sentence(word_count); // sentence: Nemo vitae rerum consequuntur vero animi incidunt esse doloribus eos.
    let data = words::paragraph(count, sentence_count, word_count, separator); // paragraph: Minima aut numquam nihil rerum commodi pariatur dolores...
    let data = words::question(); // question: Placeat voluptatem at ut eveniet suscipit similique dicta quis?
    let data = words::quote(); // quote: "Dignissimos dolorem quam tempore excepturi facere dicta." - Willy Kihn

    let opts = words::ParagraphOpts {
        count: 5,
        sentence_count: 4,
        word_count: 11,
        separator: "\n".to_string(),
    };
    let data = words::paragraph_generator(opts, &words::sentence); // paragraph_generator: Quisquam aut consequuntur nobis voluptas porro...
}
```
