// DO NOT MODIFY
// File was generated by gentz.py on 2025-01-30 11:46:10
//
//

use std::collections::HashMap;
use once_cell::sync::Lazy;

static GEO_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Africa/Harare", "CAT-2");
    m.insert("Africa/Tunis", "CET-1");
    m.insert("Africa/Nairobi", "EAT-3");
    m.insert("Africa/Kinshasa", "WAT-1");
    m.insert("Africa/El_Aaiun", "<+01>-1");
    m.insert("Africa/Nouakchott", "GMT0");
    m.insert("Africa/Kigali", "CAT-2");
    m.insert("Africa/Libreville", "WAT-1");
    m.insert("Africa/Dakar", "GMT0");
    m.insert("Africa/Ceuta", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Africa/Lagos", "WAT-1");
    m.insert("Africa/Cairo", "EET-2EEST,M4.5.5/0,M10.5.4/24");
    m.insert("Africa/Abidjan", "GMT0");
    m.insert("Africa/Bangui", "WAT-1");
    m.insert("Africa/Algiers", "CET-1");
    m.insert("Africa/Windhoek", "CAT-2");
    m.insert("Africa/Bujumbura", "CAT-2");
    m.insert("Africa/Lusaka", "CAT-2");
    m.insert("Africa/Timbuktu", "GMT0");
    m.insert("Africa/Tripoli", "EET-2");
    m.insert("Africa/Lubumbashi", "CAT-2");
    m.insert("Africa/Luanda", "WAT-1");
    m.insert("Africa/Dar_es_Salaam", "EAT-3");
    m.insert("Africa/Mbabane", "SAST-2");
    m.insert("Africa/Malabo", "WAT-1");
    m.insert("Africa/Casablanca", "<+01>-1");
    m.insert("Africa/Brazzaville", "WAT-1");
    m.insert("Africa/Lome", "GMT0");
    m.insert("Africa/Mogadishu", "EAT-3");
    m.insert("Africa/Douala", "WAT-1");
    m.insert("Africa/Addis_Ababa", "EAT-3");
    m.insert("Africa/Monrovia", "GMT0");
    m.insert("Africa/Sao_Tome", "GMT0");
    m.insert("Africa/Conakry", "GMT0");
    m.insert("Africa/Khartoum", "CAT-2");
    m.insert("Africa/Djibouti", "EAT-3");
    m.insert("Africa/Accra", "GMT0");
    m.insert("Africa/Juba", "CAT-2");
    m.insert("Africa/Gaborone", "CAT-2");
    m.insert("Africa/Blantyre", "CAT-2");
    m.insert("Africa/Niamey", "WAT-1");
    m.insert("Africa/Porto-Novo", "WAT-1");
    m.insert("Africa/Maseru", "SAST-2");
    m.insert("Africa/Kampala", "EAT-3");
    m.insert("Africa/Banjul", "GMT0");
    m.insert("Africa/Johannesburg", "SAST-2");
    m.insert("Africa/Ndjamena", "WAT-1");
    m.insert("Africa/Bissau", "GMT0");
    m.insert("Africa/Bamako", "GMT0");
    m.insert("Africa/Asmara", "EAT-3");
    m.insert("Africa/Freetown", "GMT0");
    m.insert("Africa/Ouagadougou", "GMT0");
    m.insert("Africa/Maputo", "CAT-2");
    m.insert("Arctic/Longyearbyen", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Atlantic/South_Georgia", "<-02>2");
    m.insert("Atlantic/Cape_Verde", "<-01>1");
    m.insert("Atlantic/St_Helena", "GMT0");
    m.insert("Atlantic/Canary", "WET0WEST,M3.5.0/1,M10.5.0");
    m.insert("Atlantic/Reykjavik", "GMT0");
    m.insert("Atlantic/Madeira", "WET0WEST,M3.5.0/1,M10.5.0");
    m.insert("Atlantic/Stanley", "<-03>3");
    m.insert("Atlantic/Bermuda", "AST4ADT,M3.2.0,M11.1.0");
    m.insert("Atlantic/Jan_Mayen", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Atlantic/Azores", "<-01>1<+00>,M3.5.0/0,M10.5.0/1");
    m.insert("Atlantic/Faroe", "WET0WEST,M3.5.0/1,M10.5.0");
    m.insert("Europe/Samara", "<+04>-4");
    m.insert("Europe/Stockholm", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Jersey", "GMT0BST,M3.5.0/1,M10.5.0");
    m.insert("Europe/Helsinki", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Skopje", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Oslo", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Istanbul", "<+03>-3");
    m.insert("Europe/Chisinau", "EET-2EEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Kirov", "MSK-3");
    m.insert("Europe/San_Marino", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Vaduz", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Kaliningrad", "EET-2");
    m.insert("Europe/Malta", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Busingen", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Warsaw", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Volgograd", "MSK-3");
    m.insert("Europe/Copenhagen", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Bratislava", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Tiraspol", "EET-2EEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Vatican", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Vienna", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Paris", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Nicosia", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Tirane", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Monaco", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Ulyanovsk", "<+04>-4");
    m.insert("Europe/Podgorica", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Riga", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Budapest", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Madrid", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Dublin", "IST-1GMT0,M10.5.0,M3.5.0/1");
    m.insert("Europe/Isle_of_Man", "GMT0BST,M3.5.0/1,M10.5.0");
    m.insert("Europe/Lisbon", "WET0WEST,M3.5.0/1,M10.5.0");
    m.insert("Europe/Moscow", "MSK-3");
    m.insert("Europe/Rome", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Belfast", "GMT0BST,M3.5.0/1,M10.5.0");
    m.insert("Europe/Minsk", "<+03>-3");
    m.insert("Europe/Astrakhan", "<+04>-4");
    m.insert("Europe/Tallinn", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Gibraltar", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Zurich", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Ljubljana", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Vilnius", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/London", "GMT0BST,M3.5.0/1,M10.5.0");
    m.insert("Europe/Athens", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Kyiv", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Sarajevo", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Zagreb", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Brussels", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Saratov", "<+04>-4");
    m.insert("Europe/Belgrade", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Mariehamn", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Amsterdam", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Luxembourg", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Andorra", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Berlin", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Guernsey", "GMT0BST,M3.5.0/1,M10.5.0");
    m.insert("Europe/Sofia", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Europe/Simferopol", "MSK-3");
    m.insert("Europe/Prague", "CET-1CEST,M3.5.0,M10.5.0/3");
    m.insert("Europe/Bucharest", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("America/St_Thomas", "AST4");
    m.insert("America/Boise", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/Managua", "CST6");
    m.insert("America/Lower_Princes", "AST4");
    m.insert("America/Mazatlan", "MST7");
    m.insert("America/Chicago", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Curacao", "AST4");
    m.insert("America/Yakutat", "AKST9AKDT,M3.2.0,M11.1.0");
    m.insert("America/Belem", "<-03>3");
    m.insert("America/Phoenix", "MST7");
    m.insert("America/Santarem", "<-03>3");
    m.insert("America/Danmarkshavn", "GMT0");
    m.insert("America/Rio_Branco", "<-05>5");
    m.insert("America/Cambridge_Bay", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/Asuncion", "<-04>4<-03>,M10.1.0/0,M3.4.0/0");
    m.insert("America/Nuuk", "<-02>2<-01>,M3.5.0/-1,M10.5.0/0");
    m.insert("America/Jamaica", "EST5");
    m.insert("America/Recife", "<-03>3");
    m.insert("America/Marigot", "AST4");
    m.insert("America/Cayenne", "<-03>3");
    m.insert("America/Havana", "CST5CDT,M3.2.0/0,M11.1.0/1");
    m.insert("America/St_Barthelemy", "AST4");
    m.insert("America/Matamoros", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Swift_Current", "CST6");
    m.insert("America/Thule", "AST4ADT,M3.2.0,M11.1.0");
    m.insert("America/Denver", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/Dawson", "MST7");
    m.insert("America/Rankin_Inlet", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Sao_Paulo", "<-03>3");
    m.insert("America/Rainy_River", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Santa_Isabel", "PST8PDT,M3.2.0,M11.1.0");
    m.insert("America/Mexico_City", "CST6");
    m.insert("America/Guyana", "<-04>4");
    m.insert("America/Iqaluit", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Ojinaga", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Tegucigalpa", "CST6");
    m.insert("America/Costa_Rica", "CST6");
    m.insert("America/Martinique", "AST4");
    m.insert("America/Goose_Bay", "AST4ADT,M3.2.0,M11.1.0");
    m.insert("America/Aruba", "AST4");
    m.insert("America/Shiprock", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/Guadeloupe", "AST4");
    m.insert("America/Tortola", "AST4");
    m.insert("America/Nassau", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Miquelon", "<-03>3<-02>,M3.2.0,M11.1.0");
    m.insert("America/Thunder_Bay", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Campo_Grande", "<-04>4");
    m.insert("America/Merida", "CST6");
    m.insert("America/Cuiaba", "<-04>4");
    m.insert("America/Resolute", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Ensenada", "PST8PDT,M3.2.0,M11.1.0");
    m.insert("America/Grenada", "AST4");
    m.insert("America/Sitka", "AKST9AKDT,M3.2.0,M11.1.0");
    m.insert("America/Anchorage", "AKST9AKDT,M3.2.0,M11.1.0");
    m.insert("America/Atka", "HST10HDT,M3.2.0,M11.1.0");
    m.insert("America/Santo_Domingo", "AST4");
    m.insert("America/Hermosillo", "MST7");
    m.insert("America/Barbados", "AST4");
    m.insert("America/Cancun", "EST5");
    m.insert("America/Bahia", "<-03>3");
    m.insert("America/Grand_Turk", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Boa_Vista", "<-04>4");
    m.insert("America/Moncton", "AST4ADT,M3.2.0,M11.1.0");
    m.insert("America/Cayman", "EST5");
    m.insert("America/Port_of_Spain", "AST4");
    m.insert("America/Coral_Harbour", "EST5");
    m.insert("America/Tijuana", "PST8PDT,M3.2.0,M11.1.0");
    m.insert("America/St_Lucia", "AST4");
    m.insert("America/Panama", "EST5");
    m.insert("America/Noronha", "<-02>2");
    m.insert("America/Virgin", "AST4");
    m.insert("America/Manaus", "<-04>4");
    m.insert("America/Santiago", "<-04>4<-03>,M9.1.6/24,M4.1.6/24");
    m.insert("America/Yellowknife", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/St_Johns", "NST3:30NDT,M3.2.0,M11.1.0");
    m.insert("America/Edmonton", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/Pangnirtung", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Montserrat", "AST4");
    m.insert("America/Juneau", "AKST9AKDT,M3.2.0,M11.1.0");
    m.insert("America/Eirunepe", "<-05>5");
    m.insert("America/Ciudad_Juarez", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/Bogota", "<-05>5");
    m.insert("America/Metlakatla", "AKST9AKDT,M3.2.0,M11.1.0");
    m.insert("America/Anguilla", "AST4");
    m.insert("America/Porto_Acre", "<-05>5");
    m.insert("America/Antigua", "AST4");
    m.insert("America/Detroit", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Paramaribo", "<-03>3");
    m.insert("America/Atikokan", "EST5");
    m.insert("America/Creston", "MST7");
    m.insert("America/La_Paz", "<-04>4");
    m.insert("America/Scoresbysund", "<-02>2<-01>,M3.5.0/-1,M10.5.0/0");
    m.insert("America/Fort_Nelson", "MST7");
    m.insert("America/Belize", "CST6");
    m.insert("America/Puerto_Rico", "AST4");
    m.insert("America/Dawson_Creek", "MST7");
    m.insert("America/Maceio", "<-03>3");
    m.insert("America/Toronto", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/St_Kitts", "AST4");
    m.insert("America/Glace_Bay", "AST4ADT,M3.2.0,M11.1.0");
    m.insert("America/Nome", "AKST9AKDT,M3.2.0,M11.1.0");
    m.insert("America/Dominica", "AST4");
    m.insert("America/Lima", "<-05>5");
    m.insert("America/Guayaquil", "<-05>5");
    m.insert("America/New_York", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Punta_Arenas", "<-03>3");
    m.insert("America/Winnipeg", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Nipigon", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Bahia_Banderas", "CST6");
    m.insert("America/Los_Angeles", "PST8PDT,M3.2.0,M11.1.0");
    m.insert("America/Araguaina", "<-03>3");
    m.insert("America/Porto_Velho", "<-04>4");
    m.insert("America/Adak", "HST10HDT,M3.2.0,M11.1.0");
    m.insert("America/Montevideo", "<-03>3");
    m.insert("America/Montreal", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Fortaleza", "<-03>3");
    m.insert("America/Monterrey", "CST6");
    m.insert("America/Vancouver", "PST8PDT,M3.2.0,M11.1.0");
    m.insert("America/St_Vincent", "AST4");
    m.insert("America/Whitehorse", "MST7");
    m.insert("America/Inuvik", "MST7MDT,M3.2.0,M11.1.0");
    m.insert("America/Halifax", "AST4ADT,M3.2.0,M11.1.0");
    m.insert("America/Blanc-Sablon", "AST4");
    m.insert("America/Kralendijk", "AST4");
    m.insert("America/Port-au-Prince", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Guatemala", "CST6");
    m.insert("America/Chihuahua", "CST6");
    m.insert("America/Menominee", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/El_Salvador", "CST6");
    m.insert("America/Regina", "CST6");
    m.insert("America/Caracas", "<-04>4");
    m.insert("America/Indiana/Marengo", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Indiana/Tell_City", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Indiana/Indianapolis", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Indiana/Vincennes", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Indiana/Knox", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Indiana/Petersburg", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Indiana/Vevay", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Indiana/Winamac", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Argentina/Ushuaia", "<-03>3");
    m.insert("America/Argentina/Mendoza", "<-03>3");
    m.insert("America/Argentina/Catamarca", "<-03>3");
    m.insert("America/Argentina/Rio_Gallegos", "<-03>3");
    m.insert("America/Argentina/San_Luis", "<-03>3");
    m.insert("America/Argentina/Cordoba", "<-03>3");
    m.insert("America/Argentina/Salta", "<-03>3");
    m.insert("America/Argentina/La_Rioja", "<-03>3");
    m.insert("America/Argentina/Tucuman", "<-03>3");
    m.insert("America/Argentina/Buenos_Aires", "<-03>3");
    m.insert("America/Argentina/San_Juan", "<-03>3");
    m.insert("America/Argentina/Jujuy", "<-03>3");
    m.insert("America/North_Dakota/Beulah", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/North_Dakota/Center", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/North_Dakota/New_Salem", "CST6CDT,M3.2.0,M11.1.0");
    m.insert("America/Kentucky/Louisville", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("America/Kentucky/Monticello", "EST5EDT,M3.2.0,M11.1.0");
    m.insert("Australia/Darwin", "ACST-9:30");
    m.insert("Australia/Lord_Howe", "<+1030>-10:30<+11>-11,M10.1.0,M4.1.0");
    m.insert("Australia/Adelaide", "ACST-9:30ACDT,M10.1.0,M4.1.0/3");
    m.insert("Australia/Perth", "AWST-8");
    m.insert("Australia/Sydney", "AEST-10AEDT,M10.1.0,M4.1.0/3");
    m.insert("Australia/Canberra", "AEST-10AEDT,M10.1.0,M4.1.0/3");
    m.insert("Australia/Eucla", "<+0845>-8:45");
    m.insert("Australia/Lindeman", "AEST-10");
    m.insert("Australia/Currie", "AEST-10AEDT,M10.1.0,M4.1.0/3");
    m.insert("Australia/Yancowinna", "ACST-9:30ACDT,M10.1.0,M4.1.0/3");
    m.insert("Australia/Brisbane", "AEST-10");
    m.insert("Australia/Melbourne", "AEST-10AEDT,M10.1.0,M4.1.0/3");
    m.insert("Australia/Broken_Hill", "ACST-9:30ACDT,M10.1.0,M4.1.0/3");
    m.insert("Australia/Hobart", "AEST-10AEDT,M10.1.0,M4.1.0/3");
    m.insert("Asia/Kuala_Lumpur", "<+08>-8");
    m.insert("Asia/Samarkand", "<+05>-5");
    m.insert("Asia/Kuwait", "<+03>-3");
    m.insert("Asia/Baku", "<+04>-4");
    m.insert("Asia/Almaty", "<+05>-5");
    m.insert("Asia/Novosibirsk", "<+07>-7");
    m.insert("Asia/Ust-Nera", "<+10>-10");
    m.insert("Asia/Tokyo", "JST-9");
    m.insert("Asia/Tehran", "<+0330>-3:30");
    m.insert("Asia/Atyrau", "<+05>-5");
    m.insert("Asia/Damascus", "<+03>-3");
    m.insert("Asia/Jakarta", "WIB-7");
    m.insert("Asia/Brunei", "<+08>-8");
    m.insert("Asia/Tel_Aviv", "IST-2IDT,M3.4.4/26,M10.5.0");
    m.insert("Asia/Ho_Chi_Minh", "<+07>-7");
    m.insert("Asia/Vladivostok", "<+10>-10");
    m.insert("Asia/Istanbul", "<+03>-3");
    m.insert("Asia/Dhaka", "<+06>-6");
    m.insert("Asia/Srednekolymsk", "<+11>-11");
    m.insert("Asia/Omsk", "<+06>-6");
    m.insert("Asia/Bishkek", "<+06>-6");
    m.insert("Asia/Thimphu", "<+06>-6");
    m.insert("Asia/Tbilisi", "<+04>-4");
    m.insert("Asia/Manila", "PST-8");
    m.insert("Asia/Kamchatka", "<+12>-12");
    m.insert("Asia/Krasnoyarsk", "<+07>-7");
    m.insert("Asia/Phnom_Penh", "<+07>-7");
    m.insert("Asia/Hong_Kong", "HKT-8");
    m.insert("Asia/Qostanay", "<+05>-5");
    m.insert("Asia/Aqtobe", "<+05>-5");
    m.insert("Asia/Kashgar", "<+06>-6");
    m.insert("Asia/Shanghai", "CST-8");
    m.insert("Asia/Tomsk", "<+07>-7");
    m.insert("Asia/Jerusalem", "IST-2IDT,M3.4.4/26,M10.5.0");
    m.insert("Asia/Chongqing", "CST-8");
    m.insert("Asia/Amman", "<+03>-3");
    m.insert("Asia/Khandyga", "<+09>-9");
    m.insert("Asia/Famagusta", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Asia/Nicosia", "EET-2EEST,M3.5.0/3,M10.5.0/4");
    m.insert("Asia/Dili", "<+09>-9");
    m.insert("Asia/Aqtau", "<+05>-5");
    m.insert("Asia/Irkutsk", "<+08>-8");
    m.insert("Asia/Yerevan", "<+04>-4");
    m.insert("Asia/Oral", "<+05>-5");
    m.insert("Asia/Bahrain", "<+03>-3");
    m.insert("Asia/Macau", "CST-8");
    m.insert("Asia/Ulaanbaatar", "<+08>-8");
    m.insert("Asia/Tashkent", "<+05>-5");
    m.insert("Asia/Qyzylorda", "<+05>-5");
    m.insert("Asia/Kuching", "<+08>-8");
    m.insert("Asia/Dushanbe", "<+05>-5");
    m.insert("Asia/Chita", "<+09>-9");
    m.insert("Asia/Novokuznetsk", "<+07>-7");
    m.insert("Asia/Pyongyang", "KST-9");
    m.insert("Asia/Beirut", "EET-2EEST,M3.5.0/0,M10.5.0/0");
    m.insert("Asia/Colombo", "<+0530>-5:30");
    m.insert("Asia/Riyadh", "<+03>-3");
    m.insert("Asia/Ashgabat", "<+05>-5");
    m.insert("Asia/Hovd", "<+07>-7");
    m.insert("Asia/Sakhalin", "<+11>-11");
    m.insert("Asia/Yekaterinburg", "<+05>-5");
    m.insert("Asia/Taipei", "CST-8");
    m.insert("Asia/Barnaul", "<+07>-7");
    m.insert("Asia/Bangkok", "<+07>-7");
    m.insert("Asia/Seoul", "KST-9");
    m.insert("Asia/Makassar", "WITA-8");
    m.insert("Asia/Jayapura", "WIT-9");
    m.insert("Asia/Baghdad", "<+03>-3");
    m.insert("Asia/Kathmandu", "<+0545>-5:45");
    m.insert("Asia/Hebron", "EET-2EEST,M3.4.4/50,M10.4.4/50");
    m.insert("Asia/Harbin", "CST-8");
    m.insert("Asia/Aden", "<+03>-3");
    m.insert("Asia/Dubai", "<+04>-4");
    m.insert("Asia/Qatar", "<+03>-3");
    m.insert("Asia/Kabul", "<+0430>-4:30");
    m.insert("Asia/Urumqi", "<+06>-6");
    m.insert("Asia/Muscat", "<+04>-4");
    m.insert("Asia/Singapore", "<+08>-8");
    m.insert("Asia/Choibalsan", "<+08>-8");
    m.insert("Asia/Pontianak", "WIB-7");
    m.insert("Asia/Karachi", "PKT-5");
    m.insert("Asia/Kolkata", "IST-5:30");
    m.insert("Asia/Vientiane", "<+07>-7");
    m.insert("Asia/Magadan", "<+11>-11");
    m.insert("Asia/Gaza", "EET-2EEST,M3.4.4/50,M10.4.4/50");
    m.insert("Asia/Yakutsk", "<+09>-9");
    m.insert("Asia/Yangon", "<+0630>-6:30");
    m.insert("Asia/Anadyr", "<+12>-12");
    m.insert("Pacific/Honolulu", "HST10");
    m.insert("Pacific/Samoa", "SST11");
    m.insert("Pacific/Tahiti", "<-10>10");
    m.insert("Pacific/Majuro", "<+12>-12");
    m.insert("Pacific/Kanton", "<+13>-13");
    m.insert("Pacific/Norfolk", "<+11>-11<+12>,M10.1.0,M4.1.0/3");
    m.insert("Pacific/Easter", "<-06>6<-05>,M9.1.6/22,M4.1.6/22");
    m.insert("Pacific/Auckland", "NZST-12NZDT,M9.5.0,M4.1.0/3");
    m.insert("Pacific/Apia", "<+13>-13");
    m.insert("Pacific/Pitcairn", "<-08>8");
    m.insert("Pacific/Wake", "<+12>-12");
    m.insert("Pacific/Bougainville", "<+11>-11");
    m.insert("Pacific/Kwajalein", "<+12>-12");
    m.insert("Pacific/Niue", "<-11>11");
    m.insert("Pacific/Marquesas", "<-0930>9:30");
    m.insert("Pacific/Gambier", "<-09>9");
    m.insert("Pacific/Tongatapu", "<+13>-13");
    m.insert("Pacific/Saipan", "ChST-10");
    m.insert("Pacific/Rarotonga", "<-10>10");
    m.insert("Pacific/Guadalcanal", "<+11>-11");
    m.insert("Pacific/Noumea", "<+11>-11");
    m.insert("Pacific/Palau", "<+09>-9");
    m.insert("Pacific/Efate", "<+11>-11");
    m.insert("Pacific/Midway", "SST11");
    m.insert("Pacific/Kiritimati", "<+14>-14");
    m.insert("Pacific/Wallis", "<+12>-12");
    m.insert("Pacific/Tarawa", "<+12>-12");
    m.insert("Pacific/Kosrae", "<+11>-11");
    m.insert("Pacific/Fakaofo", "<+13>-13");
    m.insert("Pacific/Funafuti", "<+12>-12");
    m.insert("Pacific/Chatham", "<+1245>-12:45<+1345>,M9.5.0/2:45,M4.1.0/3:45");
    m.insert("Pacific/Chuuk", "<+10>-10");
    m.insert("Pacific/Fiji", "<+12>-12");
    m.insert("Pacific/Johnston", "HST10");
    m.insert("Pacific/Pohnpei", "<+11>-11");
    m.insert("Pacific/Yap", "<+10>-10");
    m.insert("Pacific/Nauru", "<+12>-12");
    m.insert("Pacific/Guam", "ChST-10");
    m.insert("Pacific/Port_Moresby", "<+10>-10");
    m.insert("Pacific/Pago_Pago", "SST11");
    m.insert("Pacific/Galapagos", "<-06>6");
    m.insert("Indian/Mahe", "<+04>-4");
    m.insert("Indian/Maldives", "<+05>-5");
    m.insert("Indian/Cocos", "<+0630>-6:30");
    m.insert("Indian/Mauritius", "<+04>-4");
    m.insert("Indian/Chagos", "<+06>-6");
    m.insert("Indian/Kerguelen", "<+05>-5");
    m.insert("Indian/Antananarivo", "EAT-3");
    m.insert("Indian/Mayotte", "EAT-3");
    m.insert("Indian/Comoro", "EAT-3");
    m.insert("Indian/Reunion", "<+04>-4");
    m.insert("Indian/Christmas", "<+07>-7");
    m.insert("Antarctica/Rothera", "<-03>3");
    m.insert("Antarctica/Casey", "<+08>-8");
    m.insert("Antarctica/Macquarie", "AEST-10AEDT,M10.1.0,M4.1.0/3");
    m.insert("Antarctica/Vostok", "<+05>-5");
    m.insert("Antarctica/Troll", "<+00>0<+02>-2,M3.5.0/1,M10.5.0/3");
    m.insert("Antarctica/Mawson", "<+05>-5");
    m.insert("Antarctica/Syowa", "<+03>-3");
    m.insert("Antarctica/Davis", "<+07>-7");
    m.insert("Antarctica/McMurdo", "NZST-12NZDT,M9.5.0,M4.1.0/3");
    m.insert("Antarctica/Palmer", "<-03>3");
    m.insert("Antarctica/DumontDUrville", "<+10>-10");
    m.insert("Etc/GMT-6", "<+06>-6");
    m.insert("Etc/GMT-4", "<+04>-4");
    m.insert("Etc/GMT+4", "<-04>4");
    m.insert("Etc/GMT+11", "<-11>11");
    m.insert("Etc/GMT-8", "<+08>-8");
    m.insert("Etc/GMT+12", "<-12>12");
    m.insert("Etc/GMT", "GMT0");
    m.insert("Etc/Zulu", "UTC0");
    m.insert("Etc/GMT+6", "<-06>6");
    m.insert("Etc/GMT-9", "<+09>-9");
    m.insert("Etc/GMT+9", "<-09>9");
    m.insert("Etc/GMT-3", "<+03>-3");
    m.insert("Etc/GMT-2", "<+02>-2");
    m.insert("Etc/GMT-7", "<+07>-7");
    m.insert("Etc/GMT-13", "<+13>-13");
    m.insert("Etc/GMT-11", "<+11>-11");
    m.insert("Etc/GMT+7", "<-07>7");
    m.insert("Etc/GMT+3", "<-03>3");
    m.insert("Etc/GMT-12", "<+12>-12");
    m.insert("Etc/Universal", "UTC0");
    m.insert("Etc/GMT+1", "<-01>1");
    m.insert("Etc/GMT-1", "<+01>-1");
    m.insert("Etc/GMT0", "GMT0");
    m.insert("Etc/UTC", "UTC0");
    m.insert("Etc/GMT+0", "GMT0");
    m.insert("Etc/GMT+8", "<-08>8");
    m.insert("Etc/GMT-5", "<+05>-5");
    m.insert("Etc/GMT+2", "<-02>2");
    m.insert("Etc/GMT+5", "<-05>5");
    m.insert("Etc/GMT-0", "GMT0");
    m.insert("Etc/GMT-10", "<+10>-10");
    m.insert("Etc/Greenwich", "GMT0");
    m.insert("Etc/GMT+10", "<-10>10");
    m.insert("Etc/UCT", "UTC0");
    m.insert("Etc/GMT-14", "<+14>-14");
    m
});

pub fn lookup_tz_posix_string(key: &str) -> Option<&'static str> {
    GEO_MAP.get(key).copied()
}
