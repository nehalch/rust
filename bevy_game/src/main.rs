use mysql::*;
use mysql::prelude::*;
#[derive(Debug, PartialEq, Eq)]
struct Dev {
    country_id: i32,
    salary: i32,
    lang: Option<String>
}


fn main() -> Result<()> {
    let url = "mysql://andrew:Qwerty!23@localhost:3306/db1";
    let opts = Opts::from_url(url)?;
    let pool = mysql::Pool::new(opts)?;

    let mut conn = pool.get_conn()?;
    
    conn.query_drop(
        r"create temporary table dev (
            coUntry_id int not null,
            salary int not null,
            lang text
        )"
    )?;

    let devs = vec![
        Dev{ country_id: 1, salary: 10_000, lang: Some("python".into())},
        Dev{ country_id: 2, salary: 10_001, lang: None},
        Dev{ country_id: 3, salary: 60_000, lang: Some("rust".into())},
    ];

    conn.exec_batch(r"insert into dev (country_id, salary, lang) values(:country_id, :salary, :lang)", devs.iter().map(|p| params!{
        "country_id" => p.country_id,
        "salary" => p.salary,
        "lang" => &p.lang
    }));

    let selected_devs = conn.query_map("select country_id, salary, lang from dev", |(country_id, salary, lang)| {
        Dev { country_id, salary, lang }
    })?;

    assert_eq!(devs, selected_devs);
    Ok(())
}