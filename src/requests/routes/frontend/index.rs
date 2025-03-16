use askama::Template;
use warp::{path, Filter, Rejection, Reply};

use crate::requests::routes::frontend::common_dto::MetaProps;

pub struct IndexPageData {
    title: String,
    hero_banner: String,
    hero_banner_alt: String,
    author_picture: String,
    author_picture_alt: String,
    author_description: String,
}

#[derive(Template)]
#[template(path = "./index.html")]
pub struct IndexPage {
    meta: MetaProps,
    page: IndexPageData,
}

pub async fn get_template(locale: i8) -> Result<impl Reply, Rejection> {
    let mut page_data: IndexPageData = if locale == 0 {
        IndexPageData {
            title: String::from("Rosemary - obrazy, foto"),
            hero_banner: String::from("http://static.localhost/images/herobaner_1920.jpeg"),
            hero_banner_alt: String::from("rosemary artist hero landing page banner"),
            author_picture: String::from("http://static.localhost/images/author_home.jpeg"),
            author_picture_alt: String::from("author home page profile picture"),
            author_description: String::from(
                r#"Prací v_bance, návštěvou mnichů v_klášterech ležících v_tibetských Himalájích, mnoha osobními vzestupy a_pády, tím_vším jsem_si_v_životě prošla, než jsem nalezla útěchu a_klid ve_vyjadřování svých emocí na_malířské plátno nebo_fotografický papír.
Rytmus mé_oblíbené hudby probouzí něco v_mém nitru a_vede můj štětec.
Přestože v_mých obrazech můžete vidět mnoho věcí, nejsou to_portréty a_nejsou to_ani_krajiny, jsem_to_já."#,
            ),
        }
    } else {
        IndexPageData {
            title: String::from("Rosemary - paintings, photo"),
            hero_banner: String::from("http://static.localhost/images/herobaner_1920.jpeg"),
            hero_banner_alt: String::from("rosemary artist hero landing page banner"),
            author_picture: String::from("http://static.localhost/images/author_home.jpeg"),
            author_picture_alt: String::from("author home page profile picture"),
            author_description: String::from(
                r#"Through working_in a_bank, visiting monks in_the_Himalayas of_Nepal, experiencing multiple personal rises and falls to_finally finding a_comfort in_expressing my feelings and_emotions on_canvas or_through photography.
Passing control of_my_hands to_whatever lies deep down within my_subconsciousness, letting it flow freely in_harmony with the tunes of_my_favorite music.
It_is_not a_portrait, it_is_not a_landscape either, though people may see various things in_it, but most importantly, it_is_me."#,
            ),
        }
    };

    page_data.author_description = page_data.author_description.replace("_", "&nbsp;");

    let meta_props: MetaProps = if locale == 0 {
        MetaProps {
            description: String::from(
                "Rosemary je abstraktní malířka a fotografka žijící v Praze, Česká Republicka",
            ),
            keywords: String::from("obrazy, fotografie, foto, abstrakce, olej, umění, malování"),
            author: String::from("Rosemary - Michaela Halásová"),
            robots: String::from("index, follow"),
            image: String::from(""),
            locale: String::from("cz"),
            favicon: String::from(""),
            url: String::from("www.rosemary-artist.com/cz"),
            twitter_handle: String::from(""),
            summary_large_image: String::from(""),
        }
    } else {
        MetaProps {
            description: String::from(
                "Rosemary is abstract painter and photographer located in Prague, Czechia",
            ),
            keywords: String::from("paitings, abstract, oil, photo, family, weddings, art, paint"),
            author: String::from("Rosemary - Michaela Halásová"),
            robots: String::from("index, follow"),
            image: String::from(""),
            locale: String::from("en"),
            favicon: String::from(""),
            url: String::from("www.rosemary-artist.com/en"),
            twitter_handle: String::from(""),
            summary_large_image: String::from(""),
        }
    };

    let template = IndexPage {
        meta: meta_props,
        page: page_data,
    };

    let result = template
        .render()
        .unwrap_or_else(|_| String::from("<h1>Internal Server Error</h1>"));

    Ok(warp::reply::html(result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and_then(|| async { get_template(0).await })
}

pub fn get_cz() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("cz"))
        .and(path::end())
        .and_then(|| async { get_template(0).await })
}

pub fn get_en() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("en"))
        .and(path::end())
        .and_then(|| async { get_template(1).await })
}
