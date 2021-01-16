use actix_web::{test, App};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use parking_lot::Mutex;
use r2d2::PooledConnection;
use rand::{thread_rng, Rng};

use super::*;

static DB_LOCK: Mutex<()> = parking_lot::const_mutex(());

macro_rules! run_db_test {
    (|$conn:ident| $block:expr) => {{
        let _lock = DB_LOCK.lock();
        let $conn = create_db_conn();

        assert!(Poo::delete_all(&$conn));
        $block
    }};
}

#[actix_rt::test]
async fn visit_index_page() {
    let mut app = test::init_service(App::new().configure(app_config)).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());
}

#[actix_rt::test]
async fn insert_poo_passes() {
    run_db_test!(|conn| {
        let mut app = test::init_service(App::new().configure(app_config)).await;

        // Post poo form
        let mut rng = thread_rng();
        let params = PooInsertForm {
            form: rng.gen_range(1..6),
            color: rng.gen_range(1..6),
            bleeding: rng.gen_range(1..4),
            required_time: "00:10".to_string(),
            published_at: "2021-04-01T13:34".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/poo")
            .set_form(&params)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_redirection());

        // See flash message
        assert!(res
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("added"));

        assert_eq!(Poo::all(&conn).len(), 1);
    });
}

async fn insert_poo_fails(params: PooInsertForm) {
    run_db_test!(|conn| {
        let mut app = test::init_service(App::new().configure(app_config)).await;

        // Post poo form (invalid form)
        let req = test::TestRequest::post()
            .uri("/poo")
            .set_form(&params)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_redirection());

        // See flash message
        assert!(res
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("Failed"));

        assert_eq!(Poo::all(&conn).len(), 0);
    });
}

#[actix_rt::test]
async fn insert_poo_with_invalid_form() {
    let mut rng = thread_rng();
    insert_poo_fails(PooInsertForm {
        form: 6,
        color: rng.gen_range(1..6),
        bleeding: rng.gen_range(1..4),
        required_time: "00:10".to_string(),
        published_at: "2021-04-01T13:34".to_string(),
    })
    .await;
}

#[actix_rt::test]
async fn insert_poo_with_invalid_color() {
    let mut rng = thread_rng();
    insert_poo_fails(PooInsertForm {
        form: rng.gen_range(1..6),
        color: 6,
        bleeding: rng.gen_range(1..4),
        required_time: "00:10".to_string(),
        published_at: "2021-04-01T13:34".to_string(),
    })
    .await;
}

#[actix_rt::test]
async fn insert_poo_with_invalid_bleeding() {
    let mut rng = thread_rng();
    insert_poo_fails(PooInsertForm {
        form: rng.gen_range(1..6),
        color: rng.gen_range(1..6),
        bleeding: 4,
        required_time: "00:10".to_string(),
        published_at: "2021-04-01T13:34".to_string(),
    })
    .await;
}

#[actix_rt::test]
async fn delete_poo_works() {
    run_db_test!(|conn| {
        let mut app = test::init_service(App::new().configure(app_config)).await;

        // Post poo form
        let mut rng = thread_rng();
        let params = PooInsertForm {
            form: rng.gen_range(1..6),
            color: rng.gen_range(1..6),
            bleeding: rng.gen_range(1..4),
            required_time: "00:10".to_string(),
            published_at: "2021-04-01T13:34".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/poo")
            .set_form(&params)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_redirection());
        let poos_before = Poo::all(&conn);

        // Post to delete inserted poo
        let req = test::TestRequest::post()
            .uri(&format!("/poo/{}", poos_before[0].id))
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_redirection());
        // See flash message
        assert!(res
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("deleted"));
        // See the number of records
        let poos_after = Poo::all(&conn);
        assert_eq!(poos_after.len(), poos_before.len() - 1);
    });
}

#[actix_rt::test]
async fn delete_poo_with_invalid_id() {
    run_db_test!(|conn| {
        let mut app = test::init_service(App::new().configure(app_config)).await;

        // Post poo form
        let mut rng = thread_rng();
        let params = PooInsertForm {
            form: rng.gen_range(1..6),
            color: rng.gen_range(1..6),
            bleeding: rng.gen_range(1..4),
            required_time: "00:10".to_string(),
            published_at: "2021-04-01T13:34".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/poo")
            .set_form(&params)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_redirection());
        let poos_before = Poo::all(&conn);

        // Post to delete inserted poo with invalid ID
        let req = test::TestRequest::post()
            .uri(&format!("/poo/{}", poos_before[0].id + 1))
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_redirection());

        let poos_after = Poo::all(&conn);
        assert_eq!(poos_after.len(), poos_before.len());
    });
}

// TODO: creating DB pool by each test seems to be slow
fn create_db_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    dotenv().expect("Failed to read `.env` file");
    let mut connspec = std::env::var("DATABASE_URL").expect("env `DATABASE_URL` is empty");
    connspec.push_str("kusostat_test");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    pool.get().expect("Failed to get DB pool")
}
