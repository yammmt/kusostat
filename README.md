# kusostat

![Rust](https://github.com/yammmt/kusostat/workflows/Rust/badge.svg)

Stores your poo and pee info
## Develop

### CSS

This app uses [Bulma](https://bulma.io/) framework.

```console
cd bulma
# update `mystyles.scss` file...
sass --sourcemap=none mystyles.scss:../static/css/mystyles.css --style compressed
```

For Sass detail, please see [Bulma documents](https://bulma.io/documentation/customize/with-sass-cli/).

### Database (PostgreSQL)

First, make sure that your PostgreSQL works without errors.

Then write your database info to `.env` file, for example,

```text
DATABASE_URL=postgres://postgres:password@localhost/kusostat
```

And then run the following commands:

```console
diesel setup
diesel migration run
```
