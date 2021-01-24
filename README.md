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
DATABASE_URL=postgres://postgres:password@localhost/
```

And then run the following commands:

```console
diesel setup --database-url postgres://postgres:password@localhost/kusostat
diesel migration run --database-url postgres://postgres:password@localhost/kusostat
```

Your poo data is saved into `kusostat` database.

#### Running with Docker

The process is almost the same as the previous description.

:warning: Note that you must update the following username and password to your ones.

```console
export DATABASE_URL==postgres://postgres:password@localhost/kusostat
docker-compose up -d
diesel setup
diesel migration run
cargo run # --release
```

If you want to use PostgreSQL in your terminal, run the following command:

```console
docker-compose exec postgresql psql -U postgres log-collector
```

### Test

Before running tests, you have to run DB migration.

```console
# example URL
diesel setup --database-url "postgres://postgres:password@localhost/kusostat_test"
```

### Log

You can see the app log if you set the environment variable.

```console
RUST_LOG=info cargo r
```

## Notes

Here, I described some notes to use this app.

### Input time info in form

In short: use Google Chrome.

This app supposes that the following strings are sent by form.

| field name | string format |
|:---|:---|
| **Published at** | `2020-01-01T18:00` |
| **Required time** | `00:10` (10min) |

[MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) says that some browsers support date picker for the above form inputs.
And using its functions helps your work.

For example, in my macOS v10.16.7, Chrome 87.0.4280.88 supports both fields,
however, Firefox 84.0.1 doesn't support both of them and Safari 14.0 could send invalid data
(for example, Safari allows us to fill in random string in **Published at** form).
