//! Model for Timezone enum

use serde::{Deserialize, Serialize};

/// An [IANA time zone](https://www.iana.org/time-zones) identifier for a time zone.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Timezone {
    #[serde(rename = "Africa/Abidjan")]
    AfricaAbidjan,
    #[serde(rename = "Africa/Accra")]
    AfricaAccra,
    #[serde(rename = "Africa/Addis_Ababa")]
    AfricaAddisAbaba,
    #[serde(rename = "Africa/Algiers")]
    AfricaAlgiers,
    #[serde(rename = "Africa/Asmara")]
    AfricaAsmara,
    #[serde(rename = "Africa/Asmera")]
    AfricaAsmera,
    #[serde(rename = "Africa/Bamako")]
    AfricaBamako,
    #[serde(rename = "Africa/Bangui")]
    AfricaBangui,
    #[serde(rename = "Africa/Banjul")]
    AfricaBanjul,
    #[serde(rename = "Africa/Bissau")]
    AfricaBissau,
    #[serde(rename = "Africa/Blantyre")]
    AfricaBlantyre,
    #[serde(rename = "Africa/Brazzaville")]
    AfricaBrazzaville,
    #[serde(rename = "Africa/Bujumbura")]
    AfricaBujumbura,
    #[serde(rename = "Africa/Cairo")]
    AfricaCairo,
    #[serde(rename = "Africa/Casablanca")]
    AfricaCasablanca,
    #[serde(rename = "Africa/Ceuta")]
    AfricaCeuta,
    #[serde(rename = "Africa/Conakry")]
    AfricaConakry,
    #[serde(rename = "Africa/Dakar")]
    AfricaDakar,
    #[serde(rename = "Africa/Dar_es_Salaam")]
    AfricaDarEsSalaam,
    #[serde(rename = "Africa/Djibouti")]
    AfricaDjibouti,
    #[serde(rename = "Africa/Douala")]
    AfricaDouala,
    #[serde(rename = "Africa/El_Aaiun")]
    AfricaElAaiun,
    #[serde(rename = "Africa/Freetown")]
    AfricaFreetown,
    #[serde(rename = "Africa/Gaborone")]
    AfricaGaborone,
    #[serde(rename = "Africa/Harare")]
    AfricaHarare,
    #[serde(rename = "Africa/Johannesburg")]
    AfricaJohannesburg,
    #[serde(rename = "Africa/Juba")]
    AfricaJuba,
    #[serde(rename = "Africa/Kampala")]
    AfricaKampala,
    #[serde(rename = "Africa/Khartoum")]
    AfricaKhartoum,
    #[serde(rename = "Africa/Kigali")]
    AfricaKigali,
    #[serde(rename = "Africa/Kinshasa")]
    AfricaKinshasa,
    #[serde(rename = "Africa/Lagos")]
    AfricaLagos,
    #[serde(rename = "Africa/Libreville")]
    AfricaLibreville,
    #[serde(rename = "Africa/Lome")]
    AfricaLome,
    #[serde(rename = "Africa/Luanda")]
    AfricaLuanda,
    #[serde(rename = "Africa/Lubumbashi")]
    AfricaLubumbashi,
    #[serde(rename = "Africa/Lusaka")]
    AfricaLusaka,
    #[serde(rename = "Africa/Malabo")]
    AfricaMalabo,
    #[serde(rename = "Africa/Maputo")]
    AfricaMaputo,
    #[serde(rename = "Africa/Maseru")]
    AfricaMaseru,
    #[serde(rename = "Africa/Mbabane")]
    AfricaMbabane,
    #[serde(rename = "Africa/Mogadishu")]
    AfricaMogadishu,
    #[serde(rename = "Africa/Monrovia")]
    AfricaMonrovia,
    #[serde(rename = "Africa/Nairobi")]
    AfricaNairobi,
    #[serde(rename = "Africa/Ndjamena")]
    AfricaNdjamena,
    #[serde(rename = "Africa/Niamey")]
    AfricaNiamey,
    #[serde(rename = "Africa/Nouakchott")]
    AfricaNouakchott,
    #[serde(rename = "Africa/Ouagadougou")]
    AfricaOuagadougou,
    #[serde(rename = "Africa/Porto-Novo")]
    AfricaPortoNovo,
    #[serde(rename = "Africa/Sao_Tome")]
    AfricaSaoTome,
    #[serde(rename = "Africa/Timbuktu")]
    AfricaTimbuktu,
    #[serde(rename = "Africa/Tripoli")]
    AfricaTripoli,
    #[serde(rename = "Africa/Tunis")]
    AfricaTunis,
    #[serde(rename = "Africa/Windhoek")]
    AfricaWindhoek,
    #[serde(rename = "America/Adak")]
    AmericaAdak,
    #[serde(rename = "America/Anchorage")]
    AmericaAnchorage,
    #[serde(rename = "America/Anguilla")]
    AmericaAnguilla,
    #[serde(rename = "America/Antigua")]
    AmericaAntigua,
    #[serde(rename = "America/Araguaina")]
    AmericaAraguaina,
    #[serde(rename = "America/Argentina/Buenos_Aires")]
    AmericaArgentinaBuenosAires,
    #[serde(rename = "America/Argentina/Catamarca")]
    AmericaArgentinaCatamarca,
    #[serde(rename = "America/Argentina/ComodRivadavia")]
    AmericaArgentinaComodRivadavia,
    #[serde(rename = "America/Argentina/Cordoba")]
    AmericaArgentinaCordoba,
    #[serde(rename = "America/Argentina/Jujuy")]
    AmericaArgentinaJujuy,
    #[serde(rename = "America/Argentina/La_Rioja")]
    AmericaArgentinaLaRioja,
    #[serde(rename = "America/Argentina/Mendoza")]
    AmericaArgentinaMendoza,
    #[serde(rename = "America/Argentina/Rio_Gallegos")]
    AmericaArgentinaRioGallegos,
    #[serde(rename = "America/Argentina/Salta")]
    AmericaArgentinaSalta,
    #[serde(rename = "America/Argentina/San_Juan")]
    AmericaArgentinaSanJuan,
    #[serde(rename = "America/Argentina/San_Luis")]
    AmericaArgentinaSanLuis,
    #[serde(rename = "America/Argentina/Tucuman")]
    AmericaArgentinaTucuman,
    #[serde(rename = "America/Argentina/Ushuaia")]
    AmericaArgentinaUshuaia,
    #[serde(rename = "America/Aruba")]
    AmericaAruba,
    #[serde(rename = "America/Asuncion")]
    AmericaAsuncion,
    #[serde(rename = "America/Atikokan")]
    AmericaAtikokan,
    #[serde(rename = "America/Atka")]
    AmericaAtka,
    #[serde(rename = "America/Bahia")]
    AmericaBahia,
    #[serde(rename = "America/Bahia_Banderas")]
    AmericaBahiaBanderas,
    #[serde(rename = "America/Barbados")]
    AmericaBarbados,
    #[serde(rename = "America/Belem")]
    AmericaBelem,
    #[serde(rename = "America/Belize")]
    AmericaBelize,
    #[serde(rename = "America/Blanc-Sablon")]
    AmericaBlancSablon,
    #[serde(rename = "America/Boa_Vista")]
    AmericaBoaVista,
    #[serde(rename = "America/Bogota")]
    AmericaBogota,
    #[serde(rename = "America/Boise")]
    AmericaBoise,
    #[serde(rename = "America/Buenos_Aires")]
    AmericaBuenosAires,
    #[serde(rename = "America/Cambridge_Bay")]
    AmericaCambridgeBay,
    #[serde(rename = "America/Campo_Grande")]
    AmericaCampoGrande,
    #[serde(rename = "America/Cancun")]
    AmericaCancun,
    #[serde(rename = "America/Caracas")]
    AmericaCaracas,
    #[serde(rename = "America/Catamarca")]
    AmericaCatamarca,
    #[serde(rename = "America/Cayenne")]
    AmericaCayenne,
    #[serde(rename = "America/Cayman")]
    AmericaCayman,
    #[serde(rename = "America/Chicago")]
    AmericaChicago,
    #[serde(rename = "America/Chihuahua")]
    AmericaChihuahua,
    #[serde(rename = "America/Coral_Harbour")]
    AmericaCoralHarbour,
    #[serde(rename = "America/Cordoba")]
    AmericaCordoba,
    #[serde(rename = "America/Costa_Rica")]
    AmericaCostaRica,
    #[serde(rename = "America/Creston")]
    AmericaCreston,
    #[serde(rename = "America/Cuiaba")]
    AmericaCuiaba,
    #[serde(rename = "America/Curacao")]
    AmericaCuracao,
    #[serde(rename = "America/Danmarkshavn")]
    AmericaDanmarkshavn,
    #[serde(rename = "America/Dawson")]
    AmericaDawson,
    #[serde(rename = "America/Dawson_Creek")]
    AmericaDawsonCreek,
    #[serde(rename = "America/Denver")]
    AmericaDenver,
    #[serde(rename = "America/Detroit")]
    AmericaDetroit,
    #[serde(rename = "America/Dominica")]
    AmericaDominica,
    #[serde(rename = "America/Edmonton")]
    AmericaEdmonton,
    #[serde(rename = "America/Eirunepe")]
    AmericaEirunepe,
    #[serde(rename = "America/El_Salvador")]
    AmericaElSalvador,
    #[serde(rename = "America/Ensenada")]
    AmericaEnsenada,
    #[serde(rename = "America/Fort_Nelson")]
    AmericaFortNelson,
    #[serde(rename = "America/Fort_Wayne")]
    AmericaFortWayne,
    #[serde(rename = "America/Fortaleza")]
    AmericaFortaleza,
    #[serde(rename = "America/Glace_Bay")]
    AmericaGlaceBay,
    #[serde(rename = "America/Godthab")]
    AmericaGodthab,
    #[serde(rename = "America/Goose_Bay")]
    AmericaGooseBay,
    #[serde(rename = "America/Grand_Turk")]
    AmericaGrandTurk,
    #[serde(rename = "America/Grenada")]
    AmericaGrenada,
    #[serde(rename = "America/Guadeloupe")]
    AmericaGuadeloupe,
    #[serde(rename = "America/Guatemala")]
    AmericaGuatemala,
    #[serde(rename = "America/Guayaquil")]
    AmericaGuayaquil,
    #[serde(rename = "America/Guyana")]
    AmericaGuyana,
    #[serde(rename = "America/Halifax")]
    AmericaHalifax,
    #[serde(rename = "America/Havana")]
    AmericaHavana,
    #[serde(rename = "America/Hermosillo")]
    AmericaHermosillo,
    #[serde(rename = "America/Indiana/Indianapolis")]
    AmericaIndianaIndianapolis,
    #[serde(rename = "America/Indiana/Knox")]
    AmericaIndianaKnox,
    #[serde(rename = "America/Indiana/Marengo")]
    AmericaIndianaMarengo,
    #[serde(rename = "America/Indiana/Petersburg")]
    AmericaIndianaPetersburg,
    #[serde(rename = "America/Indiana/Tell_City")]
    AmericaIndianaTellCity,
    #[serde(rename = "America/Indiana/Vevay")]
    AmericaIndianaVevay,
    #[serde(rename = "America/Indiana/Vincennes")]
    AmericaIndianaVincennes,
    #[serde(rename = "America/Indiana/Winamac")]
    AmericaIndianaWinamac,
    #[serde(rename = "America/Indianapolis")]
    AmericaIndianapolis,
    #[serde(rename = "America/Inuvik")]
    AmericaInuvik,
    #[serde(rename = "America/Iqaluit")]
    AmericaIqaluit,
    #[serde(rename = "America/Jamaica")]
    AmericaJamaica,
    #[serde(rename = "America/Jujuy")]
    AmericaJujuy,
    #[serde(rename = "America/Juneau")]
    AmericaJuneau,
    #[serde(rename = "America/Kentucky/Louisville")]
    AmericaKentuckyLouisville,
    #[serde(rename = "America/Kentucky/Monticello")]
    AmericaKentuckyMonticello,
    #[serde(rename = "America/Knox_IN")]
    AmericaKnoxIn,
    #[serde(rename = "America/Kralendijk")]
    AmericaKralendijk,
    #[serde(rename = "America/La_Paz")]
    AmericaLaPaz,
    #[serde(rename = "America/Lima")]
    AmericaLima,
    #[serde(rename = "America/Los_Angeles")]
    AmericaLosAngeles,
    #[serde(rename = "America/Louisville")]
    AmericaLouisville,
    #[serde(rename = "America/Lower_Princes")]
    AmericaLowerPrinces,
    #[serde(rename = "America/Maceio")]
    AmericaMaceio,
    #[serde(rename = "America/Managua")]
    AmericaManagua,
    #[serde(rename = "America/Manaus")]
    AmericaManaus,
    #[serde(rename = "America/Marigot")]
    AmericaMarigot,
    #[serde(rename = "America/Martinique")]
    AmericaMartinique,
    #[serde(rename = "America/Matamoros")]
    AmericaMatamoros,
    #[serde(rename = "America/Mazatlan")]
    AmericaMazatlan,
    #[serde(rename = "America/Mendoza")]
    AmericaMendoza,
    #[serde(rename = "America/Menominee")]
    AmericaMenominee,
    #[serde(rename = "America/Merida")]
    AmericaMerida,
    #[serde(rename = "America/Metlakatla")]
    AmericaMetlakatla,
    #[serde(rename = "America/Mexico_City")]
    AmericaMexicoCity,
    #[serde(rename = "America/Miquelon")]
    AmericaMiquelon,
    #[serde(rename = "America/Moncton")]
    AmericaMoncton,
    #[serde(rename = "America/Monterrey")]
    AmericaMonterrey,
    #[serde(rename = "America/Montevideo")]
    AmericaMontevideo,
    #[serde(rename = "America/Montreal")]
    AmericaMontreal,
    #[serde(rename = "America/Montserrat")]
    AmericaMontserrat,
    #[serde(rename = "America/Nassau")]
    AmericaNassau,
    #[serde(rename = "America/New_York")]
    AmericaNewYork,
    #[serde(rename = "America/Nipigon")]
    AmericaNipigon,
    #[serde(rename = "America/Nome")]
    AmericaNome,
    #[serde(rename = "America/Noronha")]
    AmericaNoronha,
    #[serde(rename = "America/North_Dakota/Beulah")]
    AmericaNorthDakotaBeulah,
    #[serde(rename = "America/North_Dakota/Center")]
    AmericaNorthDakotaCenter,
    #[serde(rename = "America/North_Dakota/New_Salem")]
    AmericaNorthDakotaNewSalem,
    #[serde(rename = "America/Nuuk")]
    AmericaNuuk,
    #[serde(rename = "America/Ojinaga")]
    AmericaOjinaga,
    #[serde(rename = "America/Panama")]
    AmericaPanama,
    #[serde(rename = "America/Pangnirtung")]
    AmericaPangnirtung,
    #[serde(rename = "America/Paramaribo")]
    AmericaParamaribo,
    #[serde(rename = "America/Phoenix")]
    AmericaPhoenix,
    #[serde(rename = "America/Port-au-Prince")]
    AmericaPortAuPrince,
    #[serde(rename = "America/Port_of_Spain")]
    AmericaPortOfSpain,
    #[serde(rename = "America/Porto_Acre")]
    AmericaPortoAcre,
    #[serde(rename = "America/Porto_Velho")]
    AmericaPortoVelho,
    #[serde(rename = "America/Puerto_Rico")]
    AmericaPuertoRico,
    #[serde(rename = "America/Punta_Arenas")]
    AmericaPuntaArenas,
    #[serde(rename = "America/Rainy_River")]
    AmericaRainyRiver,
    #[serde(rename = "America/Rankin_Inlet")]
    AmericaRankinInlet,
    #[serde(rename = "America/Recife")]
    AmericaRecife,
    #[serde(rename = "America/Regina")]
    AmericaRegina,
    #[serde(rename = "America/Resolute")]
    AmericaResolute,
    #[serde(rename = "America/Rio_Branco")]
    AmericaRioBranco,
    #[serde(rename = "America/Rosario")]
    AmericaRosario,
    #[serde(rename = "America/Santa_Isabel")]
    AmericaSantaIsabel,
    #[serde(rename = "America/Santarem")]
    AmericaSantarem,
    #[serde(rename = "America/Santiago")]
    AmericaSantiago,
    #[serde(rename = "America/Santo_Domingo")]
    AmericaSantoDomingo,
    #[serde(rename = "America/Sao_Paulo")]
    AmericaSaoPaulo,
    #[serde(rename = "America/Scoresbysund")]
    AmericaScoresbysund,
    #[serde(rename = "America/Shiprock")]
    AmericaShiprock,
    #[serde(rename = "America/Sitka")]
    AmericaSitka,
    #[serde(rename = "America/St_Barthelemy")]
    AmericaStBarthelemy,
    #[serde(rename = "America/St_Johns")]
    AmericaStJohns,
    #[serde(rename = "America/St_Kitts")]
    AmericaStKitts,
    #[serde(rename = "America/St_Lucia")]
    AmericaStLucia,
    #[serde(rename = "America/St_Thomas")]
    AmericaStThomas,
    #[serde(rename = "America/St_Vincent")]
    AmericaStVincent,
    #[serde(rename = "America/Swift_Current")]
    AmericaSwiftCurrent,
    #[serde(rename = "America/Tegucigalpa")]
    AmericaTegucigalpa,
    #[serde(rename = "America/Thule")]
    AmericaThule,
    #[serde(rename = "America/Thunder_Bay")]
    AmericaThunderBay,
    #[serde(rename = "America/Tijuana")]
    AmericaTijuana,
    #[serde(rename = "America/Toronto")]
    AmericaToronto,
    #[serde(rename = "America/Tortola")]
    AmericaTortola,
    #[serde(rename = "America/Vancouver")]
    AmericaVancouver,
    #[serde(rename = "America/Virgin")]
    AmericaVirgin,
    #[serde(rename = "America/Whitehorse")]
    AmericaWhitehorse,
    #[serde(rename = "America/Winnipeg")]
    AmericaWinnipeg,
    #[serde(rename = "America/Yakutat")]
    AmericaYakutat,
    #[serde(rename = "America/Yellowknife")]
    AmericaYellowknife,
    #[serde(rename = "Antarctica/Casey")]
    AntarcticaCasey,
    #[serde(rename = "Antarctica/Davis")]
    AntarcticaDavis,
    #[serde(rename = "Antarctica/DumontDUrville")]
    AntarcticaDumontDUrville,
    #[serde(rename = "Antarctica/Macquarie")]
    AntarcticaMacquarie,
    #[serde(rename = "Antarctica/Mawson")]
    AntarcticaMawson,
    #[serde(rename = "Antarctica/McMurdo")]
    AntarcticaMcMurdo,
    #[serde(rename = "Antarctica/Palmer")]
    AntarcticaPalmer,
    #[serde(rename = "Antarctica/Rothera")]
    AntarcticaRothera,
    #[serde(rename = "Antarctica/South_Pole")]
    AntarcticaSouthPole,
    #[serde(rename = "Antarctica/Syowa")]
    AntarcticaSyowa,
    #[serde(rename = "Antarctica/Troll")]
    AntarcticaTroll,
    #[serde(rename = "Antarctica/Vostok")]
    AntarcticaVostok,
    #[serde(rename = "Arctic/Longyearbyen")]
    ArcticLongyearbyen,
    #[serde(rename = "Asia/Aden")]
    AsiaAden,
    #[serde(rename = "Asia/Almaty")]
    AsiaAlmaty,
    #[serde(rename = "Asia/Amman")]
    AsiaAmman,
    #[serde(rename = "Asia/Anadyr")]
    AsiaAnadyr,
    #[serde(rename = "Asia/Aqtau")]
    AsiaAqtau,
    #[serde(rename = "Asia/Aqtobe")]
    AsiaAqtobe,
    #[serde(rename = "Asia/Ashgabat")]
    AsiaAshgabat,
    #[serde(rename = "Asia/Ashkhabad")]
    AsiaAshkhabad,
    #[serde(rename = "Asia/Atyrau")]
    AsiaAtyrau,
    #[serde(rename = "Asia/Baghdad")]
    AsiaBaghdad,
    #[serde(rename = "Asia/Bahrain")]
    AsiaBahrain,
    #[serde(rename = "Asia/Baku")]
    AsiaBaku,
    #[serde(rename = "Asia/Bangkok")]
    AsiaBangkok,
    #[serde(rename = "Asia/Barnaul")]
    AsiaBarnaul,
    #[serde(rename = "Asia/Beirut")]
    AsiaBeirut,
    #[serde(rename = "Asia/Bishkek")]
    AsiaBishkek,
    #[serde(rename = "Asia/Brunei")]
    AsiaBrunei,
    #[serde(rename = "Asia/Calcutta")]
    AsiaCalcutta,
    #[serde(rename = "Asia/Chita")]
    AsiaChita,
    #[serde(rename = "Asia/Choibalsan")]
    AsiaChoibalsan,
    #[serde(rename = "Asia/Chongqing")]
    AsiaChongqing,
    #[serde(rename = "Asia/Chungking")]
    AsiaChungking,
    #[serde(rename = "Asia/Colombo")]
    AsiaColombo,
    #[serde(rename = "Asia/Dacca")]
    AsiaDacca,
    #[serde(rename = "Asia/Damascus")]
    AsiaDamascus,
    #[serde(rename = "Asia/Dhaka")]
    AsiaDhaka,
    #[serde(rename = "Asia/Dili")]
    AsiaDili,
    #[serde(rename = "Asia/Dubai")]
    AsiaDubai,
    #[serde(rename = "Asia/Dushanbe")]
    AsiaDushanbe,
    #[serde(rename = "Asia/Famagusta")]
    AsiaFamagusta,
    #[serde(rename = "Asia/Gaza")]
    AsiaGaza,
    #[serde(rename = "Asia/Harbin")]
    AsiaHarbin,
    #[serde(rename = "Asia/Hebron")]
    AsiaHebron,
    #[serde(rename = "Asia/Ho_Chi_Minh")]
    AsiaHoChiMinh,
    #[serde(rename = "Asia/Hong_Kong")]
    AsiaHongKong,
    #[serde(rename = "Asia/Hovd")]
    AsiaHovd,
    #[serde(rename = "Asia/Irkutsk")]
    AsiaIrkutsk,
    #[serde(rename = "Asia/Istanbul")]
    AsiaIstanbul,
    #[serde(rename = "Asia/Jakarta")]
    AsiaJakarta,
    #[serde(rename = "Asia/Jayapura")]
    AsiaJayapura,
    #[serde(rename = "Asia/Jerusalem")]
    AsiaJerusalem,
    #[serde(rename = "Asia/Kabul")]
    AsiaKabul,
    #[serde(rename = "Asia/Kamchatka")]
    AsiaKamchatka,
    #[serde(rename = "Asia/Karachi")]
    AsiaKarachi,
    #[serde(rename = "Asia/Kashgar")]
    AsiaKashgar,
    #[serde(rename = "Asia/Kathmandu")]
    AsiaKathmandu,
    #[serde(rename = "Asia/Katmandu")]
    AsiaKatmandu,
    #[serde(rename = "Asia/Khandyga")]
    AsiaKhandyga,
    #[serde(rename = "Asia/Kolkata")]
    AsiaKolkata,
    #[serde(rename = "Asia/Krasnoyarsk")]
    AsiaKrasnoyarsk,
    #[serde(rename = "Asia/Kuala_Lumpur")]
    AsiaKualaLumpur,
    #[serde(rename = "Asia/Kuching")]
    AsiaKuching,
    #[serde(rename = "Asia/Kuwait")]
    AsiaKuwait,
    #[serde(rename = "Asia/Macao")]
    AsiaMacao,
    #[serde(rename = "Asia/Macau")]
    AsiaMacau,
    #[serde(rename = "Asia/Magadan")]
    AsiaMagadan,
    #[serde(rename = "Asia/Makassar")]
    AsiaMakassar,
    #[serde(rename = "Asia/Manila")]
    AsiaManila,
    #[serde(rename = "Asia/Muscat")]
    AsiaMuscat,
    #[serde(rename = "Asia/Nicosia")]
    AsiaNicosia,
    #[serde(rename = "Asia/Novokuznetsk")]
    AsiaNovokuznetsk,
    #[serde(rename = "Asia/Novosibirsk")]
    AsiaNovosibirsk,
    #[serde(rename = "Asia/Omsk")]
    AsiaOmsk,
    #[serde(rename = "Asia/Oral")]
    AsiaOral,
    #[serde(rename = "Asia/Phnom_Penh")]
    AsiaPhnomPenh,
    #[serde(rename = "Asia/Pontianak")]
    AsiaPontianak,
    #[serde(rename = "Asia/Pyongyang")]
    AsiaPyongyang,
    #[serde(rename = "Asia/Qatar")]
    AsiaQatar,
    #[serde(rename = "Asia/Qostanay")]
    AsiaQostanay,
    #[serde(rename = "Asia/Qyzylorda")]
    AsiaQyzylorda,
    #[serde(rename = "Asia/Rangoon")]
    AsiaRangoon,
    #[serde(rename = "Asia/Riyadh")]
    AsiaRiyadh,
    #[serde(rename = "Asia/Saigon")]
    AsiaSaigon,
    #[serde(rename = "Asia/Sakhalin")]
    AsiaSakhalin,
    #[serde(rename = "Asia/Samarkand")]
    AsiaSamarkand,
    #[serde(rename = "Asia/Seoul")]
    AsiaSeoul,
    #[serde(rename = "Asia/Shanghai")]
    AsiaShanghai,
    #[serde(rename = "Asia/Singapore")]
    AsiaSingapore,
    #[serde(rename = "Asia/Srednekolymsk")]
    AsiaSrednekolymsk,
    #[serde(rename = "Asia/Taipei")]
    AsiaTaipei,
    #[serde(rename = "Asia/Tashkent")]
    AsiaTashkent,
    #[serde(rename = "Asia/Tbilisi")]
    AsiaTbilisi,
    #[serde(rename = "Asia/Tehran")]
    AsiaTehran,
    #[serde(rename = "Asia/Tel_Aviv")]
    AsiaTelAviv,
    #[serde(rename = "Asia/Thimbu")]
    AsiaThimbu,
    #[serde(rename = "Asia/Thimphu")]
    AsiaThimphu,
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
    #[serde(rename = "Asia/Tomsk")]
    AsiaTomsk,
    #[serde(rename = "Asia/Ujung_Pandang")]
    AsiaUjungPandang,
    #[serde(rename = "Asia/Ulaanbaatar")]
    AsiaUlaanbaatar,
    #[serde(rename = "Asia/Ulan_Bator")]
    AsiaUlanBator,
    #[serde(rename = "Asia/Urumqi")]
    AsiaUrumqi,
    #[serde(rename = "Asia/Ust-Nera")]
    AsiaUstNera,
    #[serde(rename = "Asia/Vientiane")]
    AsiaVientiane,
    #[serde(rename = "Asia/Vladivostok")]
    AsiaVladivostok,
    #[serde(rename = "Asia/Yakutsk")]
    AsiaYakutsk,
    #[serde(rename = "Asia/Yangon")]
    AsiaYangon,
    #[serde(rename = "Asia/Yekaterinburg")]
    AsiaYekaterinburg,
    #[serde(rename = "Asia/Yerevan")]
    AsiaYerevan,
    #[serde(rename = "Atlantic/Azores")]
    AtlanticAzores,
    #[serde(rename = "Atlantic/Bermuda")]
    AtlanticBermuda,
    #[serde(rename = "Atlantic/Canary")]
    AtlanticCanary,
    #[serde(rename = "Atlantic/Cape_Verde")]
    AtlanticCapeVerde,
    #[serde(rename = "Atlantic/Faeroe")]
    AtlanticFaeroe,
    #[serde(rename = "Atlantic/Faroe")]
    AtlanticFaroe,
    #[serde(rename = "Atlantic/Jan_Mayen")]
    AtlanticJanMayen,
    #[serde(rename = "Atlantic/Madeira")]
    AtlanticMadeira,
    #[serde(rename = "Atlantic/Reykjavik")]
    AtlanticReykjavik,
    #[serde(rename = "Atlantic/South_Georgia")]
    AtlanticSouthGeorgia,
    #[serde(rename = "Atlantic/St_Helena")]
    AtlanticStHelena,
    #[serde(rename = "Atlantic/Stanley")]
    AtlanticStanley,
    #[serde(rename = "Australia/ACT")]
    AustraliaACT,
    #[serde(rename = "Australia/Adelaide")]
    AustraliaAdelaide,
    #[serde(rename = "Australia/Brisbane")]
    AustraliaBrisbane,
    #[serde(rename = "Australia/Broken_Hill")]
    AustraliaBrokenHill,
    #[serde(rename = "Australia/Canberra")]
    AustraliaCanberra,
    #[serde(rename = "Australia/Currie")]
    AustraliaCurrie,
    #[serde(rename = "Australia/Darwin")]
    AustraliaDarwin,
    #[serde(rename = "Australia/Eucla")]
    AustraliaEucla,
    #[serde(rename = "Australia/Hobart")]
    AustraliaHobart,
    #[serde(rename = "Australia/LHI")]
    AustraliaLHI,
    #[serde(rename = "Australia/Lindeman")]
    AustraliaLindeman,
    #[serde(rename = "Australia/Lord_Howe")]
    AustraliaLordHowe,
    #[serde(rename = "Australia/Melbourne")]
    AustraliaMelbourne,
    #[serde(rename = "Australia/North")]
    AustraliaNorth,
    #[serde(rename = "Australia/NSW")]
    AustraliaNSW,
    #[serde(rename = "Australia/Perth")]
    AustraliaPerth,
    #[serde(rename = "Australia/Queensland")]
    AustraliaQueensland,
    #[serde(rename = "Australia/South")]
    AustraliaSouth,
    #[serde(rename = "Australia/Sydney")]
    AustraliaSydney,
    #[serde(rename = "Australia/Tasmania")]
    AustraliaTasmania,
    #[serde(rename = "Australia/Victoria")]
    AustraliaVictoria,
    #[serde(rename = "Australia/West")]
    AustraliaWest,
    #[serde(rename = "Australia/Yancowinna")]
    AustraliaYancowinna,
    #[serde(rename = "Brazil/Acre")]
    BrazilAcre,
    #[serde(rename = "Brazil/DeNoronha")]
    BrazilDeNoronha,
    #[serde(rename = "Brazil/East")]
    BrazilEast,
    #[serde(rename = "Brazil/West")]
    BrazilWest,
    #[serde(rename = "Canada/Atlantic")]
    CanadaAtlantic,
    #[serde(rename = "Canada/Central")]
    CanadaCentral,
    #[serde(rename = "Canada/Eastern")]
    CanadaEastern,
    #[serde(rename = "Canada/Mountain")]
    CanadaMountain,
    #[serde(rename = "Canada/Newfoundland")]
    CanadaNewfoundland,
    #[serde(rename = "Canada/Pacific")]
    CanadaPacific,
    #[serde(rename = "Canada/Saskatchewan")]
    CanadaSaskatchewan,
    #[serde(rename = "Canada/Yukon")]
    CanadaYukon,
    #[serde(rename = "CET")]
    CET,
    #[serde(rename = "Chile/Continental")]
    ChileContinental,
    #[serde(rename = "Chile/EasterIsland")]
    ChileEasterIsland,
    #[serde(rename = "CST6CDT")]
    CST6CDT,
    #[serde(rename = "Cuba")]
    Cuba,
    #[serde(rename = "EET")]
    EET,
    #[serde(rename = "Egypt")]
    Egypt,
    #[serde(rename = "Eire")]
    Eire,
    #[serde(rename = "EST")]
    EST,
    #[serde(rename = "EST5EDT")]
    EST5EDT,
    #[serde(rename = "Etc/GMT")]
    EtcGMT,
    #[serde(rename = "Etc/GMT+0")]
    EtcGMTPlus0,
    #[serde(rename = "Etc/GMT+1")]
    EtcGMTPlus1,
    #[serde(rename = "Etc/GMT+10")]
    EtcGMTPlus10,
    #[serde(rename = "Etc/GMT+11")]
    EtcGMTPlus11,
    #[serde(rename = "Etc/GMT+12")]
    EtcGMTPlus12,
    #[serde(rename = "Etc/GMT+2")]
    EtcGMTPlus2,
    #[serde(rename = "Etc/GMT+3")]
    EtcGMTPlus3,
    #[serde(rename = "Etc/GMT+4")]
    EtcGMTPlus4,
    #[serde(rename = "Etc/GMT+5")]
    EtcGMTPlus5,
    #[serde(rename = "Etc/GMT+6")]
    EtcGMTPlus6,
    #[serde(rename = "Etc/GMT+7")]
    EtcGMTPlus7,
    #[serde(rename = "Etc/GMT+8")]
    EtcGMTPlus8,
    #[serde(rename = "Etc/GMT+9")]
    EtcGMTPlus9,
    #[serde(rename = "Etc/GMT-0")]
    EtcGMTMinus0,
    #[serde(rename = "Etc/GMT-1")]
    EtcGMTMinus1,
    #[serde(rename = "Etc/GMT-10")]
    EtcGMTMinus10,
    #[serde(rename = "Etc/GMT-11")]
    EtcGMTMinus11,
    #[serde(rename = "Etc/GMT-12")]
    EtcGMTMinus12,
    #[serde(rename = "Etc/GMT-13")]
    EtcGMTMinus13,
    #[serde(rename = "Etc/GMT-14")]
    EtcGMTMinus14,
    #[serde(rename = "Etc/GMT-2")]
    EtcGMTMinus2,
    #[serde(rename = "Etc/GMT-3")]
    EtcGMTMinus3,
    #[serde(rename = "Etc/GMT-4")]
    EtcGMTMinus4,
    #[serde(rename = "Etc/GMT-5")]
    EtcGMTMinus5,
    #[serde(rename = "Etc/GMT-6")]
    EtcGMTMinus6,
    #[serde(rename = "Etc/GMT-7")]
    EtcGMTMinus7,
    #[serde(rename = "Etc/GMT-8")]
    EtcGMTMinus8,
    #[serde(rename = "Etc/GMT-9")]
    EtcGMTMinus9,
    #[serde(rename = "Etc/GMT0")]
    EtcGMT0,
    #[serde(rename = "Etc/Greenwich")]
    EtcGreenwich,
    #[serde(rename = "Etc/UCT")]
    EtcUCT,
    #[serde(rename = "Etc/Universal")]
    EtcUniversal,
    #[serde(rename = "Etc/UTC")]
    EtcUTC,
    #[serde(rename = "Etc/Zulu")]
    EtcZulu,
    #[serde(rename = "Europe/Amsterdam")]
    EuropeAmsterdam,
    #[serde(rename = "Europe/Andorra")]
    EuropeAndorra,
    #[serde(rename = "Europe/Astrakhan")]
    EuropeAstrakhan,
    #[serde(rename = "Europe/Athens")]
    EuropeAthens,
    #[serde(rename = "Europe/Belfast")]
    EuropeBelfast,
    #[serde(rename = "Europe/Belgrade")]
    EuropeBelgrade,
    #[serde(rename = "Europe/Berlin")]
    EuropeBerlin,
    #[serde(rename = "Europe/Bratislava")]
    EuropeBratislava,
    #[serde(rename = "Europe/Brussels")]
    EuropeBrussels,
    #[serde(rename = "Europe/Bucharest")]
    EuropeBucharest,
    #[serde(rename = "Europe/Budapest")]
    EuropeBudapest,
    #[serde(rename = "Europe/Busingen")]
    EuropeBusingen,
    #[serde(rename = "Europe/Chisinau")]
    EuropeChisinau,
    #[serde(rename = "Europe/Copenhagen")]
    EuropeCopenhagen,
    #[serde(rename = "Europe/Dublin")]
    EuropeDublin,
    #[serde(rename = "Europe/Gibraltar")]
    EuropeGibraltar,
    #[serde(rename = "Europe/Guernsey")]
    EuropeGuernsey,
    #[serde(rename = "Europe/Helsinki")]
    EuropeHelsinki,
    #[serde(rename = "Europe/Isle_of_Man")]
    EuropeIsleOfMan,
    #[serde(rename = "Europe/Istanbul")]
    EuropeIstanbul,
    #[serde(rename = "Europe/Jersey")]
    EuropeJersey,
    #[serde(rename = "Europe/Kaliningrad")]
    EuropeKaliningrad,
    #[serde(rename = "Europe/Kiev")]
    EuropeKiev,
    #[serde(rename = "Europe/Kirov")]
    EuropeKirov,
    #[serde(rename = "Europe/Lisbon")]
    EuropeLisbon,
    #[serde(rename = "Europe/Ljubljana")]
    EuropeLjubljana,
    #[serde(rename = "Europe/London")]
    EuropeLondon,
    #[serde(rename = "Europe/Luxembourg")]
    EuropeLuxembourg,
    #[serde(rename = "Europe/Madrid")]
    EuropeMadrid,
    #[serde(rename = "Europe/Malta")]
    EuropeMalta,
    #[serde(rename = "Europe/Mariehamn")]
    EuropeMariehamn,
    #[serde(rename = "Europe/Minsk")]
    EuropeMinsk,
    #[serde(rename = "Europe/Monaco")]
    EuropeMonaco,
    #[serde(rename = "Europe/Moscow")]
    EuropeMoscow,
    #[serde(rename = "Europe/Nicosia")]
    EuropeNicosia,
    #[serde(rename = "Europe/Oslo")]
    EuropeOslo,
    #[serde(rename = "Europe/Paris")]
    EuropeParis,
    #[serde(rename = "Europe/Podgorica")]
    EuropePodgorica,
    #[serde(rename = "Europe/Prague")]
    EuropePrague,
    #[serde(rename = "Europe/Riga")]
    EuropeRiga,
    #[serde(rename = "Europe/Rome")]
    EuropeRome,
    #[serde(rename = "Europe/Samara")]
    EuropeSamara,
    #[serde(rename = "Europe/San_Marino")]
    EuropeSanMarino,
    #[serde(rename = "Europe/Sarajevo")]
    EuropeSarajevo,
    #[serde(rename = "Europe/Saratov")]
    EuropeSaratov,
    #[serde(rename = "Europe/Simferopol")]
    EuropeSimferopol,
    #[serde(rename = "Europe/Skopje")]
    EuropeSkopje,
    #[serde(rename = "Europe/Sofia")]
    EuropeSofia,
    #[serde(rename = "Europe/Stockholm")]
    EuropeStockholm,
    #[serde(rename = "Europe/Tallinn")]
    EuropeTallinn,
    #[serde(rename = "Europe/Tirane")]
    EuropeTirane,
    #[serde(rename = "Europe/Tiraspol")]
    EuropeTiraspol,
    #[serde(rename = "Europe/Ulyanovsk")]
    EuropeUlyanovsk,
    #[serde(rename = "Europe/Uzhgorod")]
    EuropeUzhgorod,
    #[serde(rename = "Europe/Vaduz")]
    EuropeVaduz,
    #[serde(rename = "Europe/Vatican")]
    EuropeVatican,
    #[serde(rename = "Europe/Vienna")]
    EuropeVienna,
    #[serde(rename = "Europe/Vilnius")]
    EuropeVilnius,
    #[serde(rename = "Europe/Volgograd")]
    EuropeVolgograd,
    #[serde(rename = "Europe/Warsaw")]
    EuropeWarsaw,
    #[serde(rename = "Europe/Zagreb")]
    EuropeZagreb,
    #[serde(rename = "Europe/Zaporozhye")]
    EuropeZaporozhye,
    #[serde(rename = "Europe/Zurich")]
    EuropeZurich,
    #[serde(rename = "Factory")]
    Factory,
    #[serde(rename = "GB")]
    GB,
    #[serde(rename = "GB-Eire")]
    GBEire,
    #[serde(rename = "GMT")]
    GMT,
    #[serde(rename = "GMT+0")]
    GMTPlus0,
    #[serde(rename = "GMT-0")]
    GMTMinus0,
    #[serde(rename = "GMT0")]
    GMT0,
    #[serde(rename = "Greenwich")]
    Greenwich,
    #[serde(rename = "Hongkong")]
    Hongkong,
    #[serde(rename = "HST")]
    HST,
    #[serde(rename = "Iceland")]
    Iceland,
    #[serde(rename = "Indian/Antananarivo")]
    IndianAntananarivo,
    #[serde(rename = "Indian/Chagos")]
    IndianChagos,
    #[serde(rename = "Indian/Christmas")]
    IndianChristmas,
    #[serde(rename = "Indian/Cocos")]
    IndianCocos,
    #[serde(rename = "Indian/Comoro")]
    IndianComoro,
    #[serde(rename = "Indian/Kerguelen")]
    IndianKerguelen,
    #[serde(rename = "Indian/Mahe")]
    IndianMahe,
    #[serde(rename = "Indian/Maldives")]
    IndianMaldives,
    #[serde(rename = "Indian/Mauritius")]
    IndianMauritius,
    #[serde(rename = "Indian/Mayotte")]
    IndianMayotte,
    #[serde(rename = "Indian/Reunion")]
    IndianReunion,
    #[serde(rename = "Iran")]
    Iran,
    #[serde(rename = "Israel")]
    Israel,
    #[serde(rename = "Jamaica")]
    Jamaica,
    #[serde(rename = "Japan")]
    Japan,
    #[serde(rename = "Kwajalein")]
    Kwajalein,
    #[serde(rename = "Libya")]
    Libya,
    #[serde(rename = "MET")]
    MET,
    #[serde(rename = "Mexico/BajaNorte")]
    MexicoBajaNorte,
    #[serde(rename = "Mexico/BajaSur")]
    MexicoBajaSur,
    #[serde(rename = "Mexico/General")]
    MexicoGeneral,
    #[serde(rename = "MST")]
    MST,
    #[serde(rename = "MST7MDT")]
    MST7MDT,
    #[serde(rename = "Navajo")]
    Navajo,
    #[serde(rename = "NZ")]
    NZ,
    #[serde(rename = "NZ-CHAT")]
    NzChat,
    #[serde(rename = "Pacific/Apia")]
    PacificApia,
    #[serde(rename = "Pacific/Auckland")]
    PacificAuckland,
    #[serde(rename = "Pacific/Bougainville")]
    PacificBougainville,
    #[serde(rename = "Pacific/Chatham")]
    PacificChatham,
    #[serde(rename = "Pacific/Chuuk")]
    PacificChuuk,
    #[serde(rename = "Pacific/Easter")]
    PacificEaster,
    #[serde(rename = "Pacific/Efate")]
    PacificEfate,
    #[serde(rename = "Pacific/Enderbury")]
    PacificEnderbury,
    #[serde(rename = "Pacific/Fakaofo")]
    PacificFakaofo,
    #[serde(rename = "Pacific/Fiji")]
    PacificFiji,
    #[serde(rename = "Pacific/Funafuti")]
    PacificFunafuti,
    #[serde(rename = "Pacific/Galapagos")]
    PacificGalapagos,
    #[serde(rename = "Pacific/Gambier")]
    PacificGambier,
    #[serde(rename = "Pacific/Guadalcanal")]
    PacificGuadalcanal,
    #[serde(rename = "Pacific/Guam")]
    PacificGuam,
    #[serde(rename = "Pacific/Honolulu")]
    PacificHonolulu,
    #[serde(rename = "Pacific/Johnston")]
    PacificJohnston,
    #[serde(rename = "Pacific/Kanton")]
    PacificKanton,
    #[serde(rename = "Pacific/Kiritimati")]
    PacificKiritimati,
    #[serde(rename = "Pacific/Kosrae")]
    PacificKosrae,
    #[serde(rename = "Pacific/Kwajalein")]
    PacificKwajalein,
    #[serde(rename = "Pacific/Majuro")]
    PacificMajuro,
    #[serde(rename = "Pacific/Marquesas")]
    PacificMarquesas,
    #[serde(rename = "Pacific/Midway")]
    PacificMidway,
    #[serde(rename = "Pacific/Nauru")]
    PacificNauru,
    #[serde(rename = "Pacific/Niue")]
    PacificNiue,
    #[serde(rename = "Pacific/Norfolk")]
    PacificNorfolk,
    #[serde(rename = "Pacific/Noumea")]
    PacificNoumea,
    #[serde(rename = "Pacific/Pago_Pago")]
    PacificPagoPago,
    #[serde(rename = "Pacific/Palau")]
    PacificPalau,
    #[serde(rename = "Pacific/Pitcairn")]
    PacificPitcairn,
    #[serde(rename = "Pacific/Pohnpei")]
    PacificPohnpei,
    #[serde(rename = "Pacific/Ponape")]
    PacificPonape,
    #[serde(rename = "Pacific/Port_Moresby")]
    PacificPortMoresby,
    #[serde(rename = "Pacific/Rarotonga")]
    PacificRarotonga,
    #[serde(rename = "Pacific/Saipan")]
    PacificSaipan,
    #[serde(rename = "Pacific/Samoa")]
    PacificSamoa,
    #[serde(rename = "Pacific/Tahiti")]
    PacificTahiti,
    #[serde(rename = "Pacific/Tarawa")]
    PacificTarawa,
    #[serde(rename = "Pacific/Tongatapu")]
    PacificTongatapu,
    #[serde(rename = "Pacific/Truk")]
    PacificTruk,
    #[serde(rename = "Pacific/Wake")]
    PacificWake,
    #[serde(rename = "Pacific/Wallis")]
    PacificWallis,
    #[serde(rename = "Pacific/Yap")]
    PacificYap,
    #[serde(rename = "Poland")]
    Poland,
    #[serde(rename = "Portugal")]
    Portugal,
    #[serde(rename = "PRC")]
    Prc,
    #[serde(rename = "PST8PDT")]
    Pst8Pdt,
    #[serde(rename = "ROC")]
    Roc,
    #[serde(rename = "ROK")]
    Rok,
    #[serde(rename = "Singapore")]
    Singapore,
    #[serde(rename = "Turkey")]
    Turkey,
    #[serde(rename = "UCT")]
    Uct,
    #[serde(rename = "Universal")]
    Universal,
    #[serde(rename = "US/Alaska")]
    UsAlaska,
    #[serde(rename = "US/Aleutian")]
    UsAleutian,
    #[serde(rename = "US/Arizona")]
    UsArizona,
    #[serde(rename = "US/Central")]
    UsCentral,
    #[serde(rename = "US/East-Indiana")]
    UsEastIndiana,
    #[serde(rename = "US/Eastern")]
    UsEastern,
    #[serde(rename = "US/Hawaii")]
    UsHawaii,
    #[serde(rename = "US/Indiana-Starke")]
    UsIndianaStarke,
    #[serde(rename = "US/Michigan")]
    UsMichigan,
    #[serde(rename = "US/Mountain")]
    UsMountain,
    #[serde(rename = "US/Pacific")]
    UsPacific,
    #[serde(rename = "US/Samoa")]
    UsSamoa,
    #[serde(rename = "UTC")]
    Utc,
    #[serde(rename = "W-SU")]
    WSu,
    #[serde(rename = "WET")]
    Wet,
    #[serde(rename = "Zulu")]
    Zulu,
}

impl From<Timezone> for chrono_tz::Tz {
    fn from(val: Timezone) -> Self {
        match val {
            Timezone::AmericaLosAngeles => chrono_tz::America::Los_Angeles,
            Timezone::AmericaMonterrey => chrono_tz::America::Monterrey,
            Timezone::AmericaMontevideo => chrono_tz::America::Montevideo,
            Timezone::AmericaMontreal => chrono_tz::America::Montreal,
            Timezone::AmericaMontserrat => chrono_tz::America::Montserrat,
            Timezone::AmericaNassau => chrono_tz::America::Nassau,
            Timezone::AmericaNewYork => chrono_tz::America::New_York,
            Timezone::AmericaNuuk => chrono_tz::America::Nuuk,
            Timezone::AmericaOjinaga => chrono_tz::America::Ojinaga,
            Timezone::AmericaPanama => chrono_tz::America::Panama,
            Timezone::AmericaPangnirtung => chrono_tz::America::Pangnirtung,
            Timezone::AmericaParamaribo => chrono_tz::America::Paramaribo,
            Timezone::AmericaPhoenix => chrono_tz::America::Phoenix,
            Timezone::AmericaPortAuPrince => chrono_tz::America::PortauPrince,
            Timezone::AmericaPortOfSpain => chrono_tz::America::Port_of_Spain,
            Timezone::AmericaPortoAcre => chrono_tz::America::Porto_Acre,
            Timezone::AmericaPortoVelho => chrono_tz::America::Porto_Velho,
            Timezone::AmericaPuertoRico => chrono_tz::America::Puerto_Rico,
            Timezone::AmericaPuntaArenas => chrono_tz::America::Punta_Arenas,
            Timezone::AmericaRainyRiver => chrono_tz::America::Rainy_River,
            Timezone::AmericaRankinInlet => chrono_tz::America::Rankin_Inlet,
            Timezone::AmericaRecife => chrono_tz::America::Recife,
            Timezone::AmericaRegina => chrono_tz::America::Regina,
            Timezone::AmericaResolute => chrono_tz::America::Resolute,
            Timezone::AmericaRioBranco => chrono_tz::America::Rio_Branco,
            Timezone::AmericaRosario => chrono_tz::America::Rosario,
            Timezone::AmericaSantaIsabel => chrono_tz::America::Santa_Isabel,
            Timezone::AmericaSantarem => chrono_tz::America::Santarem,
            Timezone::AmericaSantiago => chrono_tz::America::Santiago,
            Timezone::AmericaSantoDomingo => chrono_tz::America::Santo_Domingo,
            Timezone::AmericaSaoPaulo => chrono_tz::America::Sao_Paulo,
            Timezone::AmericaScoresbysund => chrono_tz::America::Scoresbysund,
            Timezone::AmericaShiprock => chrono_tz::America::Shiprock,
            Timezone::AmericaSitka => chrono_tz::America::Sitka,
            Timezone::AmericaStBarthelemy => chrono_tz::America::St_Barthelemy,
            Timezone::AmericaStJohns => chrono_tz::America::St_Johns,
            Timezone::AmericaStKitts => chrono_tz::America::St_Kitts,
            Timezone::AmericaStLucia => chrono_tz::America::St_Lucia,
            Timezone::AmericaStThomas => chrono_tz::America::St_Thomas,
            Timezone::AmericaStVincent => chrono_tz::America::St_Vincent,
            Timezone::AmericaSwiftCurrent => chrono_tz::America::Swift_Current,
            Timezone::AmericaTegucigalpa => chrono_tz::America::Tegucigalpa,
            Timezone::AmericaThule => chrono_tz::America::Thule,
            Timezone::AmericaThunderBay => chrono_tz::America::Thunder_Bay,
            Timezone::AmericaTijuana => chrono_tz::America::Tijuana,
            Timezone::AmericaToronto => chrono_tz::America::Toronto,
            Timezone::AmericaTortola => chrono_tz::America::Tortola,
            Timezone::AmericaVancouver => chrono_tz::America::Vancouver,
            Timezone::AmericaVirgin => chrono_tz::America::Virgin,
            Timezone::AmericaWhitehorse => chrono_tz::America::Whitehorse,
            Timezone::AmericaWinnipeg => chrono_tz::America::Winnipeg,
            Timezone::AmericaYakutat => chrono_tz::America::Yakutat,
            Timezone::AmericaYellowknife => chrono_tz::America::Yellowknife,
            Timezone::Iran => chrono_tz::Iran,
            Timezone::Israel => chrono_tz::Israel,
            Timezone::Jamaica => chrono_tz::Jamaica,
            Timezone::Japan => chrono_tz::Japan,
            Timezone::Kwajalein => chrono_tz::Kwajalein,
            Timezone::Libya => chrono_tz::Libya,
            Timezone::MET => chrono_tz::MET,
            Timezone::MexicoBajaNorte => chrono_tz::Mexico::BajaNorte,
            Timezone::MexicoBajaSur => chrono_tz::Mexico::BajaSur,
            Timezone::MexicoGeneral => chrono_tz::Mexico::General,
            Timezone::MST => chrono_tz::MST,
            Timezone::MST7MDT => chrono_tz::MST7MDT,
            Timezone::Navajo => chrono_tz::Navajo,
            Timezone::NZ => chrono_tz::NZ,
            Timezone::NzChat => chrono_tz::NZCHAT,
            Timezone::PacificApia => chrono_tz::Pacific::Apia,
            Timezone::PacificAuckland => chrono_tz::Pacific::Auckland,
            Timezone::PacificBougainville => chrono_tz::Pacific::Bougainville,
            Timezone::PacificChatham => chrono_tz::Pacific::Chatham,
            Timezone::PacificChuuk => chrono_tz::Pacific::Chuuk,
            Timezone::PacificEaster => chrono_tz::Pacific::Easter,
            Timezone::PacificEfate => chrono_tz::Pacific::Efate,
            Timezone::PacificEnderbury => chrono_tz::Pacific::Enderbury,
            Timezone::PacificFakaofo => chrono_tz::Pacific::Fakaofo,
            Timezone::PacificFiji => chrono_tz::Pacific::Fiji,
            Timezone::PacificFunafuti => chrono_tz::Pacific::Funafuti,
            Timezone::PacificGalapagos => chrono_tz::Pacific::Galapagos,
            Timezone::PacificGambier => chrono_tz::Pacific::Gambier,
            Timezone::PacificGuadalcanal => chrono_tz::Pacific::Guadalcanal,
            Timezone::PacificGuam => chrono_tz::Pacific::Guam,
            Timezone::PacificHonolulu => chrono_tz::Pacific::Honolulu,
            Timezone::PacificJohnston => chrono_tz::Pacific::Johnston,
            Timezone::PacificKanton => chrono_tz::Pacific::Kanton,
            Timezone::PacificKiritimati => chrono_tz::Pacific::Kiritimati,
            Timezone::PacificKosrae => chrono_tz::Pacific::Kosrae,
            Timezone::PacificKwajalein => chrono_tz::Pacific::Kwajalein,
            Timezone::PacificMajuro => chrono_tz::Pacific::Majuro,
            Timezone::PacificMarquesas => chrono_tz::Pacific::Marquesas,
            Timezone::PacificMidway => chrono_tz::Pacific::Midway,
            Timezone::PacificNauru => chrono_tz::Pacific::Nauru,
            Timezone::PacificNiue => chrono_tz::Pacific::Niue,
            Timezone::PacificNorfolk => chrono_tz::Pacific::Norfolk,
            Timezone::PacificNoumea => chrono_tz::Pacific::Noumea,
            Timezone::PacificPagoPago => chrono_tz::Pacific::Pago_Pago,
            Timezone::PacificPalau => chrono_tz::Pacific::Palau,
            Timezone::PacificPitcairn => chrono_tz::Pacific::Pitcairn,
            Timezone::PacificPohnpei => chrono_tz::Pacific::Pohnpei,
            Timezone::PacificPonape => chrono_tz::Pacific::Ponape,
            Timezone::PacificPortMoresby => chrono_tz::Pacific::Port_Moresby,
            Timezone::PacificRarotonga => chrono_tz::Pacific::Rarotonga,
            Timezone::PacificSaipan => chrono_tz::Pacific::Saipan,
            Timezone::PacificSamoa => chrono_tz::Pacific::Samoa,
            Timezone::PacificTahiti => chrono_tz::Pacific::Tahiti,
            Timezone::PacificTarawa => chrono_tz::Pacific::Tarawa,
            Timezone::PacificTongatapu => chrono_tz::Pacific::Tongatapu,
            Timezone::PacificTruk => chrono_tz::Pacific::Truk,
            Timezone::PacificWake => chrono_tz::Pacific::Wake,
            Timezone::PacificWallis => chrono_tz::Pacific::Wallis,
            Timezone::PacificYap => chrono_tz::Pacific::Yap,
            Timezone::Poland => chrono_tz::Poland,
            Timezone::Portugal => chrono_tz::Portugal,
            Timezone::Prc => chrono_tz::PRC,
            Timezone::Pst8Pdt => chrono_tz::PST8PDT,
            Timezone::Roc => chrono_tz::ROC,
            Timezone::Rok => chrono_tz::ROK,
            Timezone::Singapore => chrono_tz::Singapore,
            Timezone::Turkey => chrono_tz::Turkey,
            Timezone::Uct => chrono_tz::UCT,
            Timezone::Universal => chrono_tz::Universal,
            Timezone::UsAlaska => chrono_tz::US::Alaska,
            Timezone::UsAleutian => chrono_tz::US::Aleutian,
            Timezone::UsArizona => chrono_tz::US::Arizona,
            Timezone::UsCentral => chrono_tz::US::Central,
            Timezone::UsEastIndiana => chrono_tz::US::EastIndiana,
            Timezone::UsEastern => chrono_tz::US::Eastern,
            Timezone::UsHawaii => chrono_tz::US::Hawaii,
            Timezone::UsIndianaStarke => chrono_tz::US::IndianaStarke,
            Timezone::UsMichigan => chrono_tz::US::Michigan,
            Timezone::UsMountain => chrono_tz::US::Mountain,
            Timezone::UsPacific => chrono_tz::US::Pacific,
            Timezone::UsSamoa => chrono_tz::US::Samoa,
            Timezone::Utc => chrono_tz::UTC,
            Timezone::WSu => chrono_tz::WSU,
            Timezone::Wet => chrono_tz::WET,
            Timezone::Zulu => chrono_tz::Zulu,
            Timezone::AfricaAbidjan => chrono_tz::Africa::Abidjan,
            Timezone::AfricaAccra => chrono_tz::Africa::Accra,
            Timezone::AfricaAddisAbaba => chrono_tz::Africa::Addis_Ababa,
            Timezone::AfricaAlgiers => chrono_tz::Africa::Algiers,
            Timezone::AfricaAsmara => chrono_tz::Africa::Asmara,
            Timezone::AfricaAsmera => chrono_tz::Africa::Asmera,
            Timezone::AfricaBamako => chrono_tz::Africa::Bamako,
            Timezone::AfricaBangui => chrono_tz::Africa::Bangui,
            Timezone::AfricaBanjul => chrono_tz::Africa::Banjul,
            Timezone::AfricaBissau => chrono_tz::Africa::Bissau,
            Timezone::AfricaBlantyre => chrono_tz::Africa::Blantyre,
            Timezone::AfricaBrazzaville => chrono_tz::Africa::Brazzaville,
            Timezone::AfricaBujumbura => chrono_tz::Africa::Bujumbura,
            Timezone::AfricaCairo => chrono_tz::Africa::Cairo,
            Timezone::AfricaCasablanca => chrono_tz::Africa::Casablanca,
            Timezone::AfricaCeuta => chrono_tz::Africa::Ceuta,
            Timezone::AfricaConakry => chrono_tz::Africa::Conakry,
            Timezone::AfricaDakar => chrono_tz::Africa::Dakar,
            Timezone::AfricaDarEsSalaam => chrono_tz::Africa::Dar_es_Salaam,
            Timezone::AfricaDjibouti => chrono_tz::Africa::Djibouti,
            Timezone::AfricaDouala => chrono_tz::Africa::Douala,
            Timezone::AfricaElAaiun => chrono_tz::Africa::El_Aaiun,
            Timezone::AfricaFreetown => chrono_tz::Africa::Freetown,
            Timezone::AfricaGaborone => chrono_tz::Africa::Gaborone,
            Timezone::AfricaHarare => chrono_tz::Africa::Harare,
            Timezone::AfricaJohannesburg => chrono_tz::Africa::Johannesburg,
            Timezone::AfricaJuba => chrono_tz::Africa::Juba,
            Timezone::AfricaKampala => chrono_tz::Africa::Kampala,
            Timezone::AfricaKhartoum => chrono_tz::Africa::Khartoum,
            Timezone::AfricaKigali => chrono_tz::Africa::Kigali,
            Timezone::AfricaKinshasa => chrono_tz::Africa::Kinshasa,
            Timezone::AfricaLagos => chrono_tz::Africa::Lagos,
            Timezone::AfricaLibreville => chrono_tz::Africa::Libreville,
            Timezone::AfricaLome => chrono_tz::Africa::Lome,
            Timezone::AfricaLuanda => chrono_tz::Africa::Luanda,
            Timezone::AfricaLubumbashi => chrono_tz::Africa::Lubumbashi,
            Timezone::AfricaLusaka => chrono_tz::Africa::Lusaka,
            Timezone::AfricaMalabo => chrono_tz::Africa::Malabo,
            Timezone::AfricaMaputo => chrono_tz::Africa::Maputo,
            Timezone::AfricaMaseru => chrono_tz::Africa::Maseru,
            Timezone::AfricaMbabane => chrono_tz::Africa::Mbabane,
            Timezone::AfricaMogadishu => chrono_tz::Africa::Mogadishu,
            Timezone::AfricaMonrovia => chrono_tz::Africa::Monrovia,
            Timezone::AfricaNairobi => chrono_tz::Africa::Nairobi,
            Timezone::AfricaNdjamena => chrono_tz::Africa::Ndjamena,
            Timezone::AfricaNiamey => chrono_tz::Africa::Niamey,
            Timezone::AfricaNouakchott => chrono_tz::Africa::Nouakchott,
            Timezone::AfricaOuagadougou => chrono_tz::Africa::Ouagadougou,
            Timezone::AfricaPortoNovo => chrono_tz::Africa::PortoNovo,
            Timezone::AfricaSaoTome => chrono_tz::Africa::Sao_Tome,
            Timezone::AfricaTimbuktu => chrono_tz::Africa::Timbuktu,
            Timezone::AfricaTripoli => chrono_tz::Africa::Tripoli,
            Timezone::AfricaTunis => chrono_tz::Africa::Tunis,
            Timezone::AfricaWindhoek => chrono_tz::Africa::Windhoek,
            Timezone::AmericaAdak => chrono_tz::America::Adak,
            Timezone::AmericaAnchorage => chrono_tz::America::Anchorage,
            Timezone::AmericaAnguilla => chrono_tz::America::Anguilla,
            Timezone::AmericaAntigua => chrono_tz::America::Antigua,
            Timezone::AmericaAraguaina => chrono_tz::America::Araguaina,
            Timezone::AmericaArgentinaBuenosAires => chrono_tz::America::Argentina::Buenos_Aires,
            Timezone::AmericaArgentinaCatamarca => chrono_tz::America::Argentina::Catamarca,
            Timezone::AmericaArgentinaComodRivadavia => {
                chrono_tz::America::Argentina::ComodRivadavia
            }
            Timezone::AmericaArgentinaCordoba => chrono_tz::America::Argentina::Cordoba,
            Timezone::AmericaArgentinaJujuy => chrono_tz::America::Argentina::Jujuy,
            Timezone::AmericaArgentinaLaRioja => chrono_tz::America::Argentina::La_Rioja,
            Timezone::AmericaArgentinaMendoza => chrono_tz::America::Argentina::Mendoza,
            Timezone::AmericaArgentinaRioGallegos => chrono_tz::America::Argentina::Rio_Gallegos,
            Timezone::AmericaArgentinaSalta => chrono_tz::America::Argentina::Salta,
            Timezone::AmericaArgentinaSanJuan => chrono_tz::America::Argentina::San_Juan,
            Timezone::AmericaArgentinaSanLuis => chrono_tz::America::Argentina::San_Luis,
            Timezone::AmericaArgentinaTucuman => chrono_tz::America::Argentina::Tucuman,
            Timezone::AmericaArgentinaUshuaia => chrono_tz::America::Argentina::Ushuaia,
            Timezone::AmericaAruba => chrono_tz::America::Aruba,
            Timezone::AmericaAsuncion => chrono_tz::America::Asuncion,
            Timezone::AmericaAtikokan => chrono_tz::America::Atikokan,
            Timezone::AmericaAtka => chrono_tz::America::Atka,
            Timezone::AmericaBahia => chrono_tz::America::Bahia,
            Timezone::AmericaBahiaBanderas => chrono_tz::America::Bahia_Banderas,
            Timezone::AmericaBarbados => chrono_tz::America::Barbados,
            Timezone::AmericaBelem => chrono_tz::America::Belem,
            Timezone::AmericaBelize => chrono_tz::America::Belize,
            Timezone::AmericaBlancSablon => chrono_tz::America::BlancSablon,
            Timezone::AmericaBoaVista => chrono_tz::America::Boa_Vista,
            Timezone::AmericaBogota => chrono_tz::America::Bogota,
            Timezone::AmericaBoise => chrono_tz::America::Boise,
            Timezone::AmericaBuenosAires => chrono_tz::America::Buenos_Aires,
            Timezone::AmericaCambridgeBay => chrono_tz::America::Cambridge_Bay,
            Timezone::AmericaCampoGrande => chrono_tz::America::Campo_Grande,
            Timezone::AmericaCancun => chrono_tz::America::Cancun,
            Timezone::AmericaCaracas => chrono_tz::America::Caracas,
            Timezone::AmericaCatamarca => chrono_tz::America::Catamarca,
            Timezone::AmericaCayenne => chrono_tz::America::Cayenne,
            Timezone::AmericaCayman => chrono_tz::America::Cayman,
            Timezone::AmericaChicago => chrono_tz::America::Chicago,
            Timezone::AmericaChihuahua => chrono_tz::America::Chihuahua,
            Timezone::AmericaCoralHarbour => chrono_tz::America::Coral_Harbour,
            Timezone::AmericaCordoba => chrono_tz::America::Cordoba,
            Timezone::AmericaCostaRica => chrono_tz::America::Costa_Rica,
            Timezone::AmericaCreston => chrono_tz::America::Creston,
            Timezone::AmericaCuiaba => chrono_tz::America::Cuiaba,
            Timezone::AmericaCuracao => chrono_tz::America::Curacao,
            Timezone::AmericaDanmarkshavn => chrono_tz::America::Danmarkshavn,
            Timezone::AmericaDawson => chrono_tz::America::Dawson,
            Timezone::AmericaDawsonCreek => chrono_tz::America::Dawson_Creek,
            Timezone::AmericaDenver => chrono_tz::America::Denver,
            Timezone::AmericaDetroit => chrono_tz::America::Detroit,
            Timezone::AmericaDominica => chrono_tz::America::Dominica,
            Timezone::AmericaEdmonton => chrono_tz::America::Edmonton,
            Timezone::AmericaEirunepe => chrono_tz::America::Eirunepe,
            Timezone::AmericaElSalvador => chrono_tz::America::El_Salvador,
            Timezone::AmericaEnsenada => chrono_tz::America::Ensenada,
            Timezone::AmericaFortNelson => chrono_tz::America::Fort_Nelson,
            Timezone::AmericaFortWayne => chrono_tz::America::Fort_Wayne,
            Timezone::AmericaFortaleza => chrono_tz::America::Fortaleza,
            Timezone::AmericaGlaceBay => chrono_tz::America::Glace_Bay,
            Timezone::AmericaGodthab => chrono_tz::America::Godthab,
            Timezone::AmericaGooseBay => chrono_tz::America::Goose_Bay,
            Timezone::AmericaGrandTurk => chrono_tz::America::Grand_Turk,
            Timezone::AmericaGrenada => chrono_tz::America::Grenada,
            Timezone::AmericaGuadeloupe => chrono_tz::America::Guadeloupe,
            Timezone::AmericaGuatemala => chrono_tz::America::Guatemala,
            Timezone::AmericaGuayaquil => chrono_tz::America::Guayaquil,
            Timezone::AmericaGuyana => chrono_tz::America::Guyana,
            Timezone::AmericaHalifax => chrono_tz::America::Halifax,
            Timezone::AmericaHavana => chrono_tz::America::Havana,
            Timezone::AmericaHermosillo => chrono_tz::America::Hermosillo,
            Timezone::AmericaIndianaIndianapolis => chrono_tz::America::Indiana::Indianapolis,
            Timezone::AmericaIndianaKnox => chrono_tz::America::Indiana::Knox,
            Timezone::AmericaIndianaMarengo => chrono_tz::America::Indiana::Marengo,
            Timezone::AmericaIndianaPetersburg => chrono_tz::America::Indiana::Petersburg,
            Timezone::AmericaIndianaTellCity => chrono_tz::America::Indiana::Tell_City,
            Timezone::AmericaIndianaVevay => chrono_tz::America::Indiana::Vevay,
            Timezone::AmericaIndianaVincennes => chrono_tz::America::Indiana::Vincennes,
            Timezone::AmericaIndianaWinamac => chrono_tz::America::Indiana::Winamac,
            Timezone::AmericaIndianapolis => chrono_tz::America::Indianapolis,
            Timezone::AmericaInuvik => chrono_tz::America::Inuvik,
            Timezone::AmericaIqaluit => chrono_tz::America::Iqaluit,
            Timezone::AmericaJamaica => chrono_tz::America::Jamaica,
            Timezone::AmericaJujuy => chrono_tz::America::Jujuy,
            Timezone::AmericaJuneau => chrono_tz::America::Juneau,
            Timezone::AmericaKentuckyLouisville => chrono_tz::America::Kentucky::Louisville,
            Timezone::AmericaKentuckyMonticello => chrono_tz::America::Kentucky::Monticello,
            Timezone::AmericaKnoxIn => chrono_tz::America::Knox_IN,
            Timezone::AmericaKralendijk => chrono_tz::America::Kralendijk,
            Timezone::AmericaLaPaz => chrono_tz::America::La_Paz,
            Timezone::AmericaLima => chrono_tz::America::Lima,
            Timezone::AmericaLouisville => chrono_tz::America::Louisville,
            Timezone::AmericaLowerPrinces => chrono_tz::America::Lower_Princes,
            Timezone::AmericaMaceio => chrono_tz::America::Maceio,
            Timezone::AmericaManagua => chrono_tz::America::Managua,
            Timezone::AmericaManaus => chrono_tz::America::Manaus,
            Timezone::AmericaMarigot => chrono_tz::America::Marigot,
            Timezone::AmericaMartinique => chrono_tz::America::Martinique,
            Timezone::AmericaMatamoros => chrono_tz::America::Matamoros,
            Timezone::AmericaMazatlan => chrono_tz::America::Mazatlan,
            Timezone::AmericaMendoza => chrono_tz::America::Mendoza,
            Timezone::AmericaMenominee => chrono_tz::America::Menominee,
            Timezone::AmericaMerida => chrono_tz::America::Merida,
            Timezone::AmericaMetlakatla => chrono_tz::America::Metlakatla,
            Timezone::AmericaMexicoCity => chrono_tz::America::Mexico_City,
            Timezone::AmericaMiquelon => chrono_tz::America::Miquelon,
            Timezone::AmericaMoncton => chrono_tz::America::Moncton,
            Timezone::AmericaNipigon => chrono_tz::America::Nipigon,
            Timezone::AmericaNome => chrono_tz::America::Nome,
            Timezone::AmericaNoronha => chrono_tz::America::Noronha,
            Timezone::AmericaNorthDakotaBeulah => chrono_tz::America::North_Dakota::Beulah,
            Timezone::AmericaNorthDakotaCenter => chrono_tz::America::North_Dakota::Center,
            Timezone::AmericaNorthDakotaNewSalem => chrono_tz::America::North_Dakota::New_Salem,
            Timezone::AntarcticaCasey => chrono_tz::Antarctica::Casey,
            Timezone::AntarcticaDavis => chrono_tz::Antarctica::Davis,
            Timezone::AntarcticaDumontDUrville => chrono_tz::Antarctica::DumontDUrville,
            Timezone::AntarcticaMacquarie => chrono_tz::Antarctica::Macquarie,
            Timezone::AntarcticaMawson => chrono_tz::Antarctica::Mawson,
            Timezone::AntarcticaMcMurdo => chrono_tz::Antarctica::McMurdo,
            Timezone::AntarcticaPalmer => chrono_tz::Antarctica::Palmer,
            Timezone::AntarcticaRothera => chrono_tz::Antarctica::Rothera,
            Timezone::AntarcticaSouthPole => chrono_tz::Antarctica::South_Pole,
            Timezone::AntarcticaSyowa => chrono_tz::Antarctica::Syowa,
            Timezone::AntarcticaTroll => chrono_tz::Antarctica::Troll,
            Timezone::AntarcticaVostok => chrono_tz::Antarctica::Vostok,
            Timezone::ArcticLongyearbyen => chrono_tz::Arctic::Longyearbyen,
            Timezone::AsiaAden => chrono_tz::Asia::Aden,
            Timezone::AsiaAlmaty => chrono_tz::Asia::Almaty,
            Timezone::AsiaAmman => chrono_tz::Asia::Amman,
            Timezone::AsiaAnadyr => chrono_tz::Asia::Anadyr,
            Timezone::AsiaAqtau => chrono_tz::Asia::Aqtau,
            Timezone::AsiaAqtobe => chrono_tz::Asia::Aqtobe,
            Timezone::AsiaAshgabat => chrono_tz::Asia::Ashgabat,
            Timezone::AsiaAshkhabad => chrono_tz::Asia::Ashkhabad,
            Timezone::AsiaAtyrau => chrono_tz::Asia::Atyrau,
            Timezone::AsiaBaghdad => chrono_tz::Asia::Baghdad,
            Timezone::AsiaBahrain => chrono_tz::Asia::Bahrain,
            Timezone::AsiaBaku => chrono_tz::Asia::Baku,
            Timezone::AsiaBangkok => chrono_tz::Asia::Bangkok,
            Timezone::AsiaBarnaul => chrono_tz::Asia::Barnaul,
            Timezone::AsiaBeirut => chrono_tz::Asia::Beirut,
            Timezone::AsiaBishkek => chrono_tz::Asia::Bishkek,
            Timezone::AsiaBrunei => chrono_tz::Asia::Brunei,
            Timezone::AsiaCalcutta => chrono_tz::Asia::Calcutta,
            Timezone::AsiaChita => chrono_tz::Asia::Chita,
            Timezone::AsiaChoibalsan => chrono_tz::Asia::Choibalsan,
            Timezone::AsiaChongqing => chrono_tz::Asia::Chongqing,
            Timezone::AsiaChungking => chrono_tz::Asia::Chungking,
            Timezone::AsiaColombo => chrono_tz::Asia::Colombo,
            Timezone::AsiaDacca => chrono_tz::Asia::Dacca,
            Timezone::AsiaDamascus => chrono_tz::Asia::Damascus,
            Timezone::AsiaDhaka => chrono_tz::Asia::Dhaka,
            Timezone::AsiaDili => chrono_tz::Asia::Dili,
            Timezone::AsiaDubai => chrono_tz::Asia::Dubai,
            Timezone::AsiaDushanbe => chrono_tz::Asia::Dushanbe,
            Timezone::AsiaFamagusta => chrono_tz::Asia::Famagusta,
            Timezone::AsiaGaza => chrono_tz::Asia::Gaza,
            Timezone::AsiaHarbin => chrono_tz::Asia::Harbin,
            Timezone::AsiaHebron => chrono_tz::Asia::Hebron,
            Timezone::AsiaHoChiMinh => chrono_tz::Asia::Ho_Chi_Minh,
            Timezone::AsiaHongKong => chrono_tz::Asia::Hong_Kong,
            Timezone::AsiaHovd => chrono_tz::Asia::Hovd,
            Timezone::AsiaIrkutsk => chrono_tz::Asia::Irkutsk,
            Timezone::AsiaIstanbul => chrono_tz::Asia::Istanbul,
            Timezone::AsiaJakarta => chrono_tz::Asia::Jakarta,
            Timezone::AsiaJayapura => chrono_tz::Asia::Jayapura,
            Timezone::AsiaJerusalem => chrono_tz::Asia::Jerusalem,
            Timezone::AsiaKabul => chrono_tz::Asia::Kabul,
            Timezone::AsiaKamchatka => chrono_tz::Asia::Kamchatka,
            Timezone::AsiaKarachi => chrono_tz::Asia::Karachi,
            Timezone::AsiaKashgar => chrono_tz::Asia::Kashgar,
            Timezone::AsiaKathmandu => chrono_tz::Asia::Kathmandu,
            Timezone::AsiaKatmandu => chrono_tz::Asia::Katmandu,
            Timezone::AsiaKhandyga => chrono_tz::Asia::Khandyga,
            Timezone::AsiaKolkata => chrono_tz::Asia::Kolkata,
            Timezone::AsiaKrasnoyarsk => chrono_tz::Asia::Krasnoyarsk,
            Timezone::AsiaKualaLumpur => chrono_tz::Asia::Kuala_Lumpur,
            Timezone::AsiaKuching => chrono_tz::Asia::Kuching,
            Timezone::AsiaKuwait => chrono_tz::Asia::Kuwait,
            Timezone::AsiaMacao => chrono_tz::Asia::Macao,
            Timezone::AsiaMacau => chrono_tz::Asia::Macau,
            Timezone::AsiaMagadan => chrono_tz::Asia::Magadan,
            Timezone::AsiaMakassar => chrono_tz::Asia::Makassar,
            Timezone::AsiaManila => chrono_tz::Asia::Manila,
            Timezone::AsiaMuscat => chrono_tz::Asia::Muscat,
            Timezone::AsiaNicosia => chrono_tz::Asia::Nicosia,
            Timezone::AsiaNovokuznetsk => chrono_tz::Asia::Novokuznetsk,
            Timezone::AsiaNovosibirsk => chrono_tz::Asia::Novosibirsk,
            Timezone::AsiaOmsk => chrono_tz::Asia::Omsk,
            Timezone::AsiaOral => chrono_tz::Asia::Oral,
            Timezone::AsiaPhnomPenh => chrono_tz::Asia::Phnom_Penh,
            Timezone::AsiaPontianak => chrono_tz::Asia::Pontianak,
            Timezone::AsiaPyongyang => chrono_tz::Asia::Pyongyang,
            Timezone::AsiaQatar => chrono_tz::Asia::Qatar,
            Timezone::AsiaQostanay => chrono_tz::Asia::Qostanay,
            Timezone::AsiaQyzylorda => chrono_tz::Asia::Qyzylorda,
            Timezone::AsiaRangoon => chrono_tz::Asia::Rangoon,
            Timezone::AsiaRiyadh => chrono_tz::Asia::Riyadh,
            Timezone::AsiaSaigon => chrono_tz::Asia::Saigon,
            Timezone::AsiaSakhalin => chrono_tz::Asia::Sakhalin,
            Timezone::AsiaSamarkand => chrono_tz::Asia::Samarkand,
            Timezone::AsiaSeoul => chrono_tz::Asia::Seoul,
            Timezone::AsiaShanghai => chrono_tz::Asia::Shanghai,
            Timezone::AsiaSingapore => chrono_tz::Asia::Singapore,
            Timezone::AsiaSrednekolymsk => chrono_tz::Asia::Srednekolymsk,
            Timezone::AsiaTaipei => chrono_tz::Asia::Taipei,
            Timezone::AsiaTashkent => chrono_tz::Asia::Tashkent,
            Timezone::AsiaTbilisi => chrono_tz::Asia::Tbilisi,
            Timezone::AsiaTehran => chrono_tz::Asia::Tehran,
            Timezone::AsiaTelAviv => chrono_tz::Asia::Tel_Aviv,
            Timezone::AsiaThimbu => chrono_tz::Asia::Thimbu,
            Timezone::AsiaThimphu => chrono_tz::Asia::Thimphu,
            Timezone::AsiaTokyo => chrono_tz::Asia::Tokyo,
            Timezone::AsiaTomsk => chrono_tz::Asia::Tomsk,
            Timezone::AsiaUjungPandang => chrono_tz::Asia::Ujung_Pandang,
            Timezone::AsiaUlaanbaatar => chrono_tz::Asia::Ulaanbaatar,
            Timezone::AsiaUlanBator => chrono_tz::Asia::Ulan_Bator,
            Timezone::AsiaUrumqi => chrono_tz::Asia::Urumqi,
            Timezone::AsiaUstNera => chrono_tz::Asia::UstNera,
            Timezone::AsiaVientiane => chrono_tz::Asia::Vientiane,
            Timezone::AsiaVladivostok => chrono_tz::Asia::Vladivostok,
            Timezone::AsiaYakutsk => chrono_tz::Asia::Yakutsk,
            Timezone::AsiaYangon => chrono_tz::Asia::Yangon,
            Timezone::AsiaYekaterinburg => chrono_tz::Asia::Yekaterinburg,
            Timezone::AsiaYerevan => chrono_tz::Asia::Yerevan,
            Timezone::AtlanticAzores => chrono_tz::Atlantic::Azores,
            Timezone::AtlanticBermuda => chrono_tz::Atlantic::Bermuda,
            Timezone::AtlanticCanary => chrono_tz::Atlantic::Canary,
            Timezone::AtlanticCapeVerde => chrono_tz::Atlantic::Cape_Verde,
            Timezone::AtlanticFaeroe => chrono_tz::Atlantic::Faeroe,
            Timezone::AtlanticFaroe => chrono_tz::Atlantic::Faroe,
            Timezone::AtlanticJanMayen => chrono_tz::Atlantic::Jan_Mayen,
            Timezone::AtlanticMadeira => chrono_tz::Atlantic::Madeira,
            Timezone::AtlanticReykjavik => chrono_tz::Atlantic::Reykjavik,
            Timezone::AtlanticSouthGeorgia => chrono_tz::Atlantic::South_Georgia,
            Timezone::AtlanticStHelena => chrono_tz::Atlantic::St_Helena,
            Timezone::AtlanticStanley => chrono_tz::Atlantic::Stanley,
            Timezone::AustraliaACT => chrono_tz::Australia::ACT,
            Timezone::AustraliaAdelaide => chrono_tz::Australia::Adelaide,
            Timezone::AustraliaBrisbane => chrono_tz::Australia::Brisbane,
            Timezone::AustraliaBrokenHill => chrono_tz::Australia::Broken_Hill,
            Timezone::AustraliaCanberra => chrono_tz::Australia::Canberra,
            Timezone::AustraliaCurrie => chrono_tz::Australia::Currie,
            Timezone::AustraliaDarwin => chrono_tz::Australia::Darwin,
            Timezone::AustraliaEucla => chrono_tz::Australia::Eucla,
            Timezone::AustraliaHobart => chrono_tz::Australia::Hobart,
            Timezone::AustraliaLHI => chrono_tz::Australia::LHI,
            Timezone::AustraliaLindeman => chrono_tz::Australia::Lindeman,
            Timezone::AustraliaLordHowe => chrono_tz::Australia::Lord_Howe,
            Timezone::AustraliaMelbourne => chrono_tz::Australia::Melbourne,
            Timezone::AustraliaNorth => chrono_tz::Australia::North,
            Timezone::AustraliaNSW => chrono_tz::Australia::NSW,
            Timezone::AustraliaPerth => chrono_tz::Australia::Perth,
            Timezone::AustraliaQueensland => chrono_tz::Australia::Queensland,
            Timezone::AustraliaSouth => chrono_tz::Australia::South,
            Timezone::AustraliaSydney => chrono_tz::Australia::Sydney,
            Timezone::AustraliaTasmania => chrono_tz::Australia::Tasmania,
            Timezone::AustraliaVictoria => chrono_tz::Australia::Victoria,
            Timezone::AustraliaWest => chrono_tz::Australia::West,
            Timezone::AustraliaYancowinna => chrono_tz::Australia::Yancowinna,
            Timezone::BrazilAcre => chrono_tz::Brazil::Acre,
            Timezone::BrazilDeNoronha => chrono_tz::Brazil::DeNoronha,
            Timezone::BrazilEast => chrono_tz::Brazil::East,
            Timezone::BrazilWest => chrono_tz::Brazil::West,
            Timezone::CanadaAtlantic => chrono_tz::Canada::Atlantic,
            Timezone::CanadaCentral => chrono_tz::Canada::Central,
            Timezone::CanadaEastern => chrono_tz::Canada::Eastern,
            Timezone::CanadaMountain => chrono_tz::Canada::Mountain,
            Timezone::CanadaNewfoundland => chrono_tz::Canada::Newfoundland,
            Timezone::CanadaPacific => chrono_tz::Canada::Pacific,
            Timezone::CanadaSaskatchewan => chrono_tz::Canada::Saskatchewan,
            Timezone::CanadaYukon => chrono_tz::Canada::Yukon,
            Timezone::CET => chrono_tz::CET,
            Timezone::ChileContinental => chrono_tz::Chile::Continental,
            Timezone::ChileEasterIsland => chrono_tz::Chile::EasterIsland,
            Timezone::CST6CDT => chrono_tz::CST6CDT,
            Timezone::Cuba => chrono_tz::Cuba,
            Timezone::EET => chrono_tz::EET,
            Timezone::Egypt => chrono_tz::Egypt,
            Timezone::Eire => chrono_tz::Eire,
            Timezone::EST => chrono_tz::EST,
            Timezone::EST5EDT => chrono_tz::EST5EDT,
            Timezone::EtcGMT => chrono_tz::Etc::GMT,
            Timezone::EtcGMTPlus0 => chrono_tz::Etc::GMTPlus0,
            Timezone::EtcGMTPlus1 => chrono_tz::Etc::GMTPlus1,
            Timezone::EtcGMTPlus10 => chrono_tz::Etc::GMTPlus10,
            Timezone::EtcGMTPlus11 => chrono_tz::Etc::GMTPlus11,
            Timezone::EtcGMTPlus12 => chrono_tz::Etc::GMTPlus12,
            Timezone::EtcGMTPlus2 => chrono_tz::Etc::GMTPlus2,
            Timezone::EtcGMTPlus3 => chrono_tz::Etc::GMTPlus3,
            Timezone::EtcGMTPlus4 => chrono_tz::Etc::GMTPlus4,
            Timezone::EtcGMTPlus5 => chrono_tz::Etc::GMTPlus5,
            Timezone::EtcGMTPlus6 => chrono_tz::Etc::GMTPlus6,
            Timezone::EtcGMTPlus7 => chrono_tz::Etc::GMTPlus7,
            Timezone::EtcGMTPlus8 => chrono_tz::Etc::GMTPlus8,
            Timezone::EtcGMTPlus9 => chrono_tz::Etc::GMTPlus9,
            Timezone::EtcGMTMinus0 => chrono_tz::Etc::GMTMinus0,
            Timezone::EtcGMTMinus1 => chrono_tz::Etc::GMTMinus1,
            Timezone::EtcGMTMinus10 => chrono_tz::Etc::GMTMinus10,
            Timezone::EtcGMTMinus11 => chrono_tz::Etc::GMTMinus11,
            Timezone::EtcGMTMinus12 => chrono_tz::Etc::GMTMinus12,
            Timezone::EtcGMTMinus13 => chrono_tz::Etc::GMTMinus13,
            Timezone::EtcGMTMinus14 => chrono_tz::Etc::GMTMinus14,
            Timezone::EtcGMTMinus2 => chrono_tz::Etc::GMTMinus2,
            Timezone::EtcGMTMinus3 => chrono_tz::Etc::GMTMinus3,
            Timezone::EtcGMTMinus4 => chrono_tz::Etc::GMTMinus4,
            Timezone::EtcGMTMinus5 => chrono_tz::Etc::GMTMinus5,
            Timezone::EtcGMTMinus6 => chrono_tz::Etc::GMTMinus6,
            Timezone::EtcGMTMinus7 => chrono_tz::Etc::GMTMinus7,
            Timezone::EtcGMTMinus8 => chrono_tz::Etc::GMTMinus8,
            Timezone::EtcGMTMinus9 => chrono_tz::Etc::GMTMinus9,
            Timezone::EtcGMT0 => chrono_tz::Etc::GMT0,
            Timezone::EtcGreenwich => chrono_tz::Etc::Greenwich,
            Timezone::EtcUCT => chrono_tz::Etc::UCT,
            Timezone::EtcUniversal => chrono_tz::Etc::Universal,
            Timezone::EtcUTC => chrono_tz::Etc::UTC,
            Timezone::EtcZulu => chrono_tz::Etc::Zulu,
            Timezone::EuropeAmsterdam => chrono_tz::Europe::Amsterdam,
            Timezone::EuropeAndorra => chrono_tz::Europe::Andorra,
            Timezone::EuropeAstrakhan => chrono_tz::Europe::Astrakhan,
            Timezone::EuropeAthens => chrono_tz::Europe::Athens,
            Timezone::EuropeBelfast => chrono_tz::Europe::Belfast,
            Timezone::EuropeBelgrade => chrono_tz::Europe::Belgrade,
            Timezone::EuropeBerlin => chrono_tz::Europe::Berlin,
            Timezone::EuropeBratislava => chrono_tz::Europe::Bratislava,
            Timezone::EuropeBrussels => chrono_tz::Europe::Brussels,
            Timezone::EuropeBucharest => chrono_tz::Europe::Bucharest,
            Timezone::EuropeBudapest => chrono_tz::Europe::Budapest,
            Timezone::EuropeBusingen => chrono_tz::Europe::Busingen,
            Timezone::EuropeChisinau => chrono_tz::Europe::Chisinau,
            Timezone::EuropeCopenhagen => chrono_tz::Europe::Copenhagen,
            Timezone::EuropeDublin => chrono_tz::Europe::Dublin,
            Timezone::EuropeGibraltar => chrono_tz::Europe::Gibraltar,
            Timezone::EuropeGuernsey => chrono_tz::Europe::Guernsey,
            Timezone::EuropeHelsinki => chrono_tz::Europe::Helsinki,
            Timezone::EuropeIsleOfMan => chrono_tz::Europe::Isle_of_Man,
            Timezone::EuropeIstanbul => chrono_tz::Europe::Istanbul,
            Timezone::EuropeJersey => chrono_tz::Europe::Jersey,
            Timezone::EuropeKaliningrad => chrono_tz::Europe::Kaliningrad,
            Timezone::EuropeKiev => chrono_tz::Europe::Kiev,
            Timezone::EuropeKirov => chrono_tz::Europe::Kirov,
            Timezone::EuropeLisbon => chrono_tz::Europe::Lisbon,
            Timezone::EuropeLjubljana => chrono_tz::Europe::Ljubljana,
            Timezone::EuropeLondon => chrono_tz::Europe::London,
            Timezone::EuropeLuxembourg => chrono_tz::Europe::Luxembourg,
            Timezone::EuropeMadrid => chrono_tz::Europe::Madrid,
            Timezone::EuropeMalta => chrono_tz::Europe::Malta,
            Timezone::EuropeMariehamn => chrono_tz::Europe::Mariehamn,
            Timezone::EuropeMinsk => chrono_tz::Europe::Minsk,
            Timezone::EuropeMonaco => chrono_tz::Europe::Monaco,
            Timezone::EuropeMoscow => chrono_tz::Europe::Moscow,
            Timezone::EuropeNicosia => chrono_tz::Europe::Nicosia,
            Timezone::EuropeOslo => chrono_tz::Europe::Oslo,
            Timezone::EuropeParis => chrono_tz::Europe::Paris,
            Timezone::EuropePodgorica => chrono_tz::Europe::Podgorica,
            Timezone::EuropePrague => chrono_tz::Europe::Prague,
            Timezone::EuropeRiga => chrono_tz::Europe::Riga,
            Timezone::EuropeRome => chrono_tz::Europe::Rome,
            Timezone::EuropeSamara => chrono_tz::Europe::Samara,
            Timezone::EuropeSanMarino => chrono_tz::Europe::San_Marino,
            Timezone::EuropeSarajevo => chrono_tz::Europe::Sarajevo,
            Timezone::EuropeSaratov => chrono_tz::Europe::Saratov,
            Timezone::EuropeSimferopol => chrono_tz::Europe::Simferopol,
            Timezone::EuropeSkopje => chrono_tz::Europe::Skopje,
            Timezone::EuropeSofia => chrono_tz::Europe::Sofia,
            Timezone::EuropeStockholm => chrono_tz::Europe::Stockholm,
            Timezone::EuropeTallinn => chrono_tz::Europe::Tallinn,
            Timezone::EuropeTirane => chrono_tz::Europe::Tirane,
            Timezone::EuropeTiraspol => chrono_tz::Europe::Tiraspol,
            Timezone::EuropeUlyanovsk => chrono_tz::Europe::Ulyanovsk,
            Timezone::EuropeUzhgorod => chrono_tz::Europe::Uzhgorod,
            Timezone::EuropeVaduz => chrono_tz::Europe::Vaduz,
            Timezone::EuropeVatican => chrono_tz::Europe::Vatican,
            Timezone::EuropeVienna => chrono_tz::Europe::Vienna,
            Timezone::EuropeVilnius => chrono_tz::Europe::Vilnius,
            Timezone::EuropeVolgograd => chrono_tz::Europe::Volgograd,
            Timezone::EuropeWarsaw => chrono_tz::Europe::Warsaw,
            Timezone::EuropeZagreb => chrono_tz::Europe::Zagreb,
            Timezone::EuropeZaporozhye => chrono_tz::Europe::Zaporozhye,
            Timezone::EuropeZurich => chrono_tz::Europe::Zurich,
            Timezone::GB => chrono_tz::GB,
            Timezone::GBEire => chrono_tz::GBEire,
            Timezone::GMT => chrono_tz::GMT,
            Timezone::GMTPlus0 => chrono_tz::GMTPlus0,
            Timezone::GMTMinus0 => chrono_tz::GMTMinus0,
            Timezone::GMT0 => chrono_tz::GMT0,
            Timezone::Greenwich => chrono_tz::Greenwich,
            Timezone::Hongkong => chrono_tz::Hongkong,
            Timezone::HST => chrono_tz::HST,
            Timezone::Iceland => chrono_tz::Iceland,
            Timezone::IndianAntananarivo => chrono_tz::Indian::Antananarivo,
            Timezone::IndianChagos => chrono_tz::Indian::Chagos,
            Timezone::IndianChristmas => chrono_tz::Indian::Christmas,
            Timezone::IndianCocos => chrono_tz::Indian::Cocos,
            Timezone::IndianComoro => chrono_tz::Indian::Comoro,
            Timezone::IndianKerguelen => chrono_tz::Indian::Kerguelen,
            Timezone::IndianMahe => chrono_tz::Indian::Mahe,
            Timezone::IndianMaldives => chrono_tz::Indian::Maldives,
            Timezone::IndianMauritius => chrono_tz::Indian::Mauritius,
            Timezone::IndianMayotte => chrono_tz::Indian::Mayotte,
            Timezone::IndianReunion => chrono_tz::Indian::Reunion,
            Timezone::Factory => chrono_tz::GMT0,
        }
    }
}
