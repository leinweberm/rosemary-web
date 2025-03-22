pub struct PageTranslation {
    pub en: &'static str,
    pub cs: &'static str,
}

#[derive(Copy, Debug, Clone)]
pub enum Language {
    En,
    Cs,
}

impl Language {
    pub fn to_string(&self) -> &str {
        match self {
            Language::Cs => "cs",
            Language::En => "en",
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub enum TranslationKeys {
    Ascending,
    Created,
    Contact,
    Descending,
    Height,
    IndexTitle,
    IndexHeroAlt,
    IndexPicturAlt,
    IndexDescription,
    IndexMetaDescription,
    IndexMetaKeywords,
    IndexMetaImageSummary,
    Navigation,
    Price,
    Sold,
    Title,
    Width,
    None,
}

pub struct PageTranslations {
    ascending: PageTranslation,
    created: PageTranslation,
    contact: PageTranslation,
    descending: PageTranslation,
    height: PageTranslation,
    index_title: PageTranslation,
    index_hero_alt: PageTranslation,
    index_picture_alt: PageTranslation,
    index_description: PageTranslation,
    index_meta_description: PageTranslation,
    index_meta_keywords: PageTranslation,
    index_meta_image_summary: PageTranslation,
    navigation: PageTranslation,
    price: PageTranslation,
    sold: PageTranslation,
    title: PageTranslation,
    width: PageTranslation,
    none: PageTranslation,
}

impl PageTranslations {
    pub fn get_translation(&self, key: TranslationKeys, lang: Language) -> &'static str {
        let translation = match key {
            TranslationKeys::Ascending => &self.ascending,
            TranslationKeys::Created => &self.created,
            TranslationKeys::Contact => &self.contact,
            TranslationKeys::Descending => &self.descending,
            TranslationKeys::Height => &self.height,
            TranslationKeys::IndexTitle => &self.index_title,
            TranslationKeys::IndexHeroAlt => &self.index_hero_alt,
            TranslationKeys::IndexPicturAlt => &self.index_picture_alt,
            TranslationKeys::IndexDescription => &self.index_description,
            TranslationKeys::IndexMetaDescription => &self.index_meta_description,
            TranslationKeys::IndexMetaKeywords => &self.index_meta_keywords,
            TranslationKeys::IndexMetaImageSummary => &self.index_meta_image_summary,
            TranslationKeys::Navigation => &self.navigation,
            TranslationKeys::Price => &self.price,
            TranslationKeys::Sold => &self.sold,
            TranslationKeys::Title => &self.title,
            TranslationKeys::Width => &self.width,
            TranslationKeys::None => &self.none,
        };

        match lang {
            Language::En => translation.en,
            Language::Cs => translation.cs,
        }
    }
}

static PAGE_TRANSLATIONS: PageTranslations = PageTranslations {
    ascending: PageTranslation {
        en: "ascending",
        cs: "vzestupně",
    },
    created: PageTranslation {
        en: "created",
        cs: "vytvořeno",
    },
    contact: PageTranslation {
        en: "contact",
        cs: "kontakt",
    },
    descending: PageTranslation {
        en: "descending",
        cs: "sestupně",
    },
    height: PageTranslation {
        en: "height",
        cs: "výška",
    },
    index_title: PageTranslation {
        en: "Rosemary - paintings, photo",
        cs: "Rosemary - obrazy, foto",
    },
    index_hero_alt: PageTranslation {
        en: "rosemary artist hero landing page banner",
        cs: "hlavní stránka webových stránek výtvarnice rosemary",
    },
    index_picture_alt: PageTranslation {
        en: "author home page profile picture",
        cs: "profilový obrázek autorky rosemary",
    },
    index_description: PageTranslation {
        en: r#"Through working_in a_bank, visiting monks in_the_Himalayas of_Nepal, experiencing multiple personal rises and falls to_finally finding a_comfort in_expressing my feelings and_emotions on_canvas or_through photography.
Passing control of_my_hands to_whatever lies deep down within my_subconsciousness, letting it flow freely in_harmony with the tunes of_my_favorite music.
It_is_not a_portrait, it_is_not a_landscape either, though people may see various things in_it, but most importantly, it_is_me."#,
        cs: r#"Prací v_bance, návštěvou mnichů v_klášterech ležících v_tibetských Himalájích, mnoha osobními vzestupy a_pády, tím_vším jsem_si_v_životě prošla, než jsem nalezla útěchu a_klid ve_vyjadřování svých emocí na_malířské plátno nebo_fotografický papír.
Rytmus mé_oblíbené hudby probouzí něco v_mém nitru a_vede můj štětec.
Přestože v_mých obrazech můžete vidět mnoho věcí, nejsou to_portréty a_nejsou to_ani_krajiny, jsem_to_já."#,
    },
    index_meta_description: PageTranslation {
        en: "Rosemary is abstract painter and photographer located in Prague, Czechia",
        cs: "Rosemary je abstraktní malířka a fotografka žijící v Praze, Česká Republicka",
    },
    index_meta_keywords: PageTranslation {
        en: "paitings, abstract, oil, photo, family, weddings, art, paint",
        cs: "obrazy, fotografie, foto, abstrakce, olej, umění, malování",
    },
    index_meta_image_summary: PageTranslation { en: "", cs: "" },
    navigation: PageTranslation {
        en: "navigation",
        cs: "navigace",
    },
    price: PageTranslation {
        en: "price",
        cs: "cena",
    },
    sold: PageTranslation {
        en: "sold",
        cs: "prodáno",
    },
    title: PageTranslation {
        en: "title",
        cs: "název",
    },
    width: PageTranslation {
        en: "width",
        cs: "šířka",
    },
    none: PageTranslation {
        en: "- none -",
        cs: "- none -",
    },
};

pub fn get_translation(key: TranslationKeys, lang: Language) -> &'static str {
    PAGE_TRANSLATIONS.get_translation(key, lang)
}
