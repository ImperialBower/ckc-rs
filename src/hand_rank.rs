use core::cmp::Ordering;
use core::fmt;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// `HandRank` represents the value of a specific 5 card hand of poker. The lower the
/// `HandRankValue` the better the hand. When a `HandRank` is instantiated it can only
/// have a specific matching `HandRankName` and `HandRankValue`.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HandRank {
    pub value: HandRankValue,
    pub name: HandRankName,
    pub class: HandRankClass,
}

impl HandRank {
    /// Takes in a calculated `HandRankValue` and returns the `HandRank`.
    ///
    /// 7462 possible combination of hands:
    ///
    ///   10 straight-flushes
    ///  156 four of a kinds
    ///  156 full houses
    /// 1277 flushes
    ///   10 straights
    ///  858 three of a kinds
    ///  858 two pairs
    /// 2860 pairs
    /// 1277 high cards
    #[must_use]
    pub fn determine_name(hrv: &HandRankValue) -> HandRankName {
        match *hrv {
            1..=10 => HandRankName::StraightFlush,
            11..=166 => HandRankName::FourOfAKind,
            167..=322 => HandRankName::FullHouse,
            323..=1599 => HandRankName::Flush,
            1600..=1609 => HandRankName::Straight,
            1610..=2467 => HandRankName::ThreeOfAKind,
            2468..=3325 => HandRankName::TwoPair,
            3326..=6185 => HandRankName::Pair,
            6186..=7462 => HandRankName::HighCard,
            _ => HandRankName::Invalid,
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn determine_class(hrv: &HandRankValue) -> HandRankClass {
        match *hrv {
            1 => HandRankClass::RoyalFlush,
            2 => HandRankClass::KingHighStraightFlush,
            3 => HandRankClass::QueenHighStraightFlush,
            4 => HandRankClass::JackHighStraightFlush,
            5 => HandRankClass::TenHighStraightFlush,
            6 => HandRankClass::NineHighStraightFlush,
            7 => HandRankClass::EightHighStraightFlush,
            8 => HandRankClass::SevenHighStraightFlush,
            9 => HandRankClass::SixHighStraightFlush,
            10 => HandRankClass::FiveHighStraightFlush,
            11..=22 => HandRankClass::FourAces,
            23..=34 => HandRankClass::FourKings,
            35..=46 => HandRankClass::FourQueens,
            47..=58 => HandRankClass::FourJacks,
            59..=70 => HandRankClass::FourTens,
            71..=82 => HandRankClass::FourNines,
            83..=94 => HandRankClass::FourEights,
            95..=106 => HandRankClass::FourSevens,
            107..=118 => HandRankClass::FourSixes,
            119..=130 => HandRankClass::FourFives,
            131..=142 => HandRankClass::FourFours,
            143..=154 => HandRankClass::FourTreys,
            155..=166 => HandRankClass::FourDeuces,
            167 => HandRankClass::AcesOverKings,
            168 => HandRankClass::AcesOverQueens,
            169 => HandRankClass::AcesOverJacks,
            170 => HandRankClass::AcesOverTens,
            171 => HandRankClass::AcesOverNines,
            172 => HandRankClass::AcesOverEights,
            173 => HandRankClass::AcesOverSevens,
            174 => HandRankClass::AcesOverSixes,
            175 => HandRankClass::AcesOverFives,
            176 => HandRankClass::AcesOverFours,
            177 => HandRankClass::AcesOverTreys,
            178 => HandRankClass::AcesOverDeuces,
            179 => HandRankClass::KingsOverAces,
            180 => HandRankClass::KingsOverQueens,
            181 => HandRankClass::KingsOverJacks,
            182 => HandRankClass::KingsOverTens,
            183 => HandRankClass::KingsOverNines,
            184 => HandRankClass::KingsOverEights,
            185 => HandRankClass::KingsOverSevens,
            186 => HandRankClass::KingsOverSixes,
            187 => HandRankClass::KingsOverFives,
            188 => HandRankClass::KingsOverFours,
            189 => HandRankClass::KingsOverTreys,
            190 => HandRankClass::KingsOverDeuces,
            191 => HandRankClass::QueensOverAces,
            192 => HandRankClass::QueensOverKings,
            193 => HandRankClass::QueensOverJacks,
            194 => HandRankClass::QueensOverTens,
            195 => HandRankClass::QueensOverNines,
            196 => HandRankClass::QueensOverEights,
            197 => HandRankClass::QueensOverSevens,
            198 => HandRankClass::QueensOverSixes,
            199 => HandRankClass::QueensOverFives,
            200 => HandRankClass::QueensOverFours,
            201 => HandRankClass::QueensOverTreys,
            202 => HandRankClass::QueensOverDeuces,
            203 => HandRankClass::JacksOverAces,
            204 => HandRankClass::JacksOverKings,
            205 => HandRankClass::JacksOverQueens,
            206 => HandRankClass::JacksOverTens,
            207 => HandRankClass::JacksOverNines,
            208 => HandRankClass::JacksOverEights,
            209 => HandRankClass::JacksOverSevens,
            210 => HandRankClass::JacksOverSixes,
            211 => HandRankClass::JacksOverFives,
            212 => HandRankClass::JacksOverFours,
            213 => HandRankClass::JacksOverTreys,
            214 => HandRankClass::JacksOverDeuces,
            215 => HandRankClass::TensOverAces,
            216 => HandRankClass::TensOverKings,
            217 => HandRankClass::TensOverQueens,
            218 => HandRankClass::TensOverJacks,
            219 => HandRankClass::TensOverNines,
            220 => HandRankClass::TensOverEights,
            221 => HandRankClass::TensOverSevens,
            222 => HandRankClass::TensOverSixes,
            223 => HandRankClass::TensOverFives,
            224 => HandRankClass::TensOverFours,
            225 => HandRankClass::TensOverTreys,
            226 => HandRankClass::TensOverDeuces,
            227 => HandRankClass::NinesOverAces,
            228 => HandRankClass::NinesOverKings,
            229 => HandRankClass::NinesOverQueens,
            230 => HandRankClass::NinesOverJacks,
            231 => HandRankClass::NinesOverTens,
            232 => HandRankClass::NinesOverEights,
            233 => HandRankClass::NinesOverSevens,
            234 => HandRankClass::NinesOverSixes,
            235 => HandRankClass::NinesOverFives,
            236 => HandRankClass::NinesOverFours,
            237 => HandRankClass::NinesOverTreys,
            238 => HandRankClass::NinesOverDeuces,
            239 => HandRankClass::EightsOverAces,
            240 => HandRankClass::EightsOverKings,
            241 => HandRankClass::EightsOverQueens,
            242 => HandRankClass::EightsOverJacks,
            243 => HandRankClass::EightsOverTens,
            244 => HandRankClass::EightsOverNines,
            245 => HandRankClass::EightsOverSevens,
            246 => HandRankClass::EightsOverSixes,
            247 => HandRankClass::EightsOverFives,
            248 => HandRankClass::EightsOverFours,
            249 => HandRankClass::EightsOverTreys,
            250 => HandRankClass::EightsOverDeuces,
            251 => HandRankClass::SevensOverAces,
            252 => HandRankClass::SevensOverKings,
            253 => HandRankClass::SevensOverQueens,
            254 => HandRankClass::SevensOverJacks,
            255 => HandRankClass::SevensOverTens,
            256 => HandRankClass::SevensOverNines,
            257 => HandRankClass::SevensOverEights,
            258 => HandRankClass::SevensOverSixes,
            259 => HandRankClass::SevensOverFives,
            260 => HandRankClass::SevensOverFours,
            261 => HandRankClass::SevensOverTreys,
            262 => HandRankClass::SevensOverDeuces,
            263 => HandRankClass::SixesOverAces,
            264 => HandRankClass::SixesOverKings,
            265 => HandRankClass::SixesOverQueens,
            266 => HandRankClass::SixesOverJacks,
            267 => HandRankClass::SixesOverTens,
            268 => HandRankClass::SixesOverNines,
            269 => HandRankClass::SixesOverEights,
            270 => HandRankClass::SixesOverSevens,
            271 => HandRankClass::SixesOverFives,
            272 => HandRankClass::SixesOverFours,
            273 => HandRankClass::SixesOverTreys,
            274 => HandRankClass::SixesOverDeuces,
            275 => HandRankClass::FivesOverAces,
            276 => HandRankClass::FivesOverKings,
            277 => HandRankClass::FivesOverQueens,
            278 => HandRankClass::FivesOverJacks,
            279 => HandRankClass::FivesOverTens,
            280 => HandRankClass::FivesOverNines,
            281 => HandRankClass::FivesOverEights,
            282 => HandRankClass::FivesOverSevens,
            283 => HandRankClass::FivesOverSixes,
            284 => HandRankClass::FivesOverFours,
            285 => HandRankClass::FivesOverTreys,
            286 => HandRankClass::FivesOverDeuces,
            287 => HandRankClass::FoursOverAces,
            288 => HandRankClass::FoursOverKings,
            289 => HandRankClass::FoursOverQueens,
            290 => HandRankClass::FoursOverJacks,
            291 => HandRankClass::FoursOverTens,
            292 => HandRankClass::FoursOverNines,
            293 => HandRankClass::FoursOverEights,
            294 => HandRankClass::FoursOverSevens,
            295 => HandRankClass::FoursOverSixes,
            296 => HandRankClass::FoursOverFives,
            297 => HandRankClass::FoursOverTreys,
            298 => HandRankClass::FoursOverDeuces,
            299 => HandRankClass::TreysOverAces,
            300 => HandRankClass::TreysOverKings,
            301 => HandRankClass::TreysOverQueens,
            302 => HandRankClass::TreysOverJacks,
            303 => HandRankClass::TreysOverTens,
            304 => HandRankClass::TreysOverNines,
            305 => HandRankClass::TreysOverEights,
            306 => HandRankClass::TreysOverSevens,
            307 => HandRankClass::TreysOverSixes,
            308 => HandRankClass::TreysOverFives,
            309 => HandRankClass::TreysOverFours,
            310 => HandRankClass::TreysOverDeuces,
            311 => HandRankClass::DeucesOverAces,
            312 => HandRankClass::DeucesOverKings,
            313 => HandRankClass::DeucesOverQueens,
            314 => HandRankClass::DeucesOverJacks,
            315 => HandRankClass::DeucesOverTens,
            316 => HandRankClass::DeucesOverNines,
            317 => HandRankClass::DeucesOverEights,
            318 => HandRankClass::DeucesOverSevens,
            319 => HandRankClass::DeucesOverSixes,
            320 => HandRankClass::DeucesOverFives,
            321 => HandRankClass::DeucesOverFours,
            322 => HandRankClass::DeucesOverTreys,
            323..=815 => HandRankClass::AceHighFlush,
            816..=1144 => HandRankClass::KingHighFlush,
            1145..=1353 => HandRankClass::QueenHighFlush,
            1354..=1478 => HandRankClass::JackHighFlush,
            1479..=1547 => HandRankClass::TenHighFlush,
            1548..=1581 => HandRankClass::NineHighFlush,
            1582..=1595 => HandRankClass::EightHighFlush,
            1596..=1599 => HandRankClass::SevenHighFlush,
            1600 => HandRankClass::AceHighStraight,
            1601 => HandRankClass::KingHighStraight,
            1602 => HandRankClass::QueenHighStraight,
            1603 => HandRankClass::JackHighStraight,
            1604 => HandRankClass::TenHighStraight,
            1605 => HandRankClass::NineHighStraight,
            1606 => HandRankClass::EightHighStraight,
            1607 => HandRankClass::SevenHighStraight,
            1608 => HandRankClass::SixHighStraight,
            1609 => HandRankClass::FiveHighStraight,
            1610..=1675 => HandRankClass::ThreeAces,
            1676..=1741 => HandRankClass::ThreeKings,
            1742..=1807 => HandRankClass::ThreeQueens,
            1808..=1873 => HandRankClass::ThreeJacks,
            1874..=1939 => HandRankClass::ThreeTens,
            1940..=2005 => HandRankClass::ThreeNines,
            2006..=2071 => HandRankClass::ThreeEights,
            2072..=2137 => HandRankClass::ThreeSevens,
            2138..=2203 => HandRankClass::ThreeSixes,
            2204..=2269 => HandRankClass::ThreeFives,
            2270..=2335 => HandRankClass::ThreeFours,
            2336..=2401 => HandRankClass::ThreeTreys,
            2402..=2467 => HandRankClass::ThreeDeuces,
            2468..=2478 => HandRankClass::AcesAndKings,
            2479..=2489 => HandRankClass::AcesAndQueens,
            2490..=2500 => HandRankClass::AcesAndJacks,
            2501..=2511 => HandRankClass::AcesAndTens,
            2512..=2522 => HandRankClass::AcesAndNines,
            2523..=2533 => HandRankClass::AcesAndEights,
            2534..=2544 => HandRankClass::AcesAndSevens,
            2545..=2555 => HandRankClass::AcesAndSixes,
            2556..=2566 => HandRankClass::AcesAndFives,
            2567..=2577 => HandRankClass::AcesAndFours,
            2578..=2588 => HandRankClass::AcesAndTreys,
            2589..=2599 => HandRankClass::AcesAndDeuces,
            2600..=2610 => HandRankClass::KingsAndQueens,
            2611..=2621 => HandRankClass::KingsAndJacks,
            2622..=2632 => HandRankClass::KingsAndTens,
            2633..=2643 => HandRankClass::KingsAndNines,
            2644..=2654 => HandRankClass::KingsAndEights,
            2655..=2665 => HandRankClass::KingsAndSevens,
            2666..=2676 => HandRankClass::KingsAndSixes,
            2677..=2687 => HandRankClass::KingsAndFives,
            2688..=2698 => HandRankClass::KingsAndFours,
            2699..=2709 => HandRankClass::KingsAndTreys,
            2710..=2720 => HandRankClass::KingsAndDeuces,
            2721..=2731 => HandRankClass::QueensAndJacks,
            2732..=2742 => HandRankClass::QueensAndTens,
            2743..=2753 => HandRankClass::QueensAndNines,
            2754..=2764 => HandRankClass::QueensAndEights,
            2765..=2775 => HandRankClass::QueensAndSevens,
            2776..=2786 => HandRankClass::QueensAndSixes,
            2787..=2797 => HandRankClass::QueensAndFives,
            2798..=2808 => HandRankClass::QueensAndFours,
            2809..=2819 => HandRankClass::QueensAndTreys,
            2820..=2830 => HandRankClass::QueensAndDeuces,
            2831..=2841 => HandRankClass::JacksAndTens,
            2842..=2852 => HandRankClass::JacksAndNines,
            2853..=2863 => HandRankClass::JacksAndEights,
            2864..=2874 => HandRankClass::JacksAndSevens,
            2875..=2885 => HandRankClass::JacksAndSixes,
            2886..=2896 => HandRankClass::JacksAndFives,
            2897..=2907 => HandRankClass::JacksAndFours,
            2908..=2918 => HandRankClass::JacksAndTreys,
            2919..=2929 => HandRankClass::JacksAndDeuces,
            2930..=2940 => HandRankClass::TensAndNines,
            2941..=2951 => HandRankClass::TensAndEights,
            2952..=2962 => HandRankClass::TensAndSevens,
            2963..=2973 => HandRankClass::TensAndSixes,
            2974..=2984 => HandRankClass::TensAndFives,
            2985..=2995 => HandRankClass::TensAndFours,
            2996..=3006 => HandRankClass::TensAndTreys,
            3007..=3017 => HandRankClass::TensAndDeuces,
            3018..=3028 => HandRankClass::NinesAndEights,
            3029..=3039 => HandRankClass::NinesAndSevens,
            3040..=3050 => HandRankClass::NinesAndSixes,
            3051..=3061 => HandRankClass::NinesAndFives,
            3062..=3072 => HandRankClass::NinesAndFours,
            3073..=3083 => HandRankClass::NinesAndTreys,
            3084..=3094 => HandRankClass::NinesAndDeuces,
            3095..=3105 => HandRankClass::EightsAndSevens,
            3106..=3116 => HandRankClass::EightsAndSixes,
            3117..=3127 => HandRankClass::EightsAndFives,
            3128..=3138 => HandRankClass::EightsAndFours,
            3139..=3149 => HandRankClass::EightsAndTreys,
            3150..=3160 => HandRankClass::EightsAndDeuces,
            3161..=3171 => HandRankClass::SevensAndSixes,
            3172..=3182 => HandRankClass::SevensAndFives,
            3183..=3193 => HandRankClass::SevensAndFours,
            3194..=3204 => HandRankClass::SevensAndTreys,
            3205..=3215 => HandRankClass::SevensAndDeuces,
            3216..=3226 => HandRankClass::SixesAndFives,
            3227..=3237 => HandRankClass::SixesAndFours,
            3238..=3248 => HandRankClass::SixesAndTreys,
            3249..=3259 => HandRankClass::SixesAndDeuces,
            3260..=3270 => HandRankClass::FivesAndFours,
            3271..=3281 => HandRankClass::FivesAndTreys,
            3282..=3292 => HandRankClass::FivesAndDeuces,
            3293..=3303 => HandRankClass::FoursAndTreys,
            3304..=3314 => HandRankClass::FoursAndDeuces,
            3315..=3325 => HandRankClass::TreysAndDeuces,
            3326..=3545 => HandRankClass::PairOfAces,
            3546..=3765 => HandRankClass::PairOfKings,
            3766..=3985 => HandRankClass::PairOfQueens,
            3986..=4205 => HandRankClass::PairOfJacks,
            4206..=4425 => HandRankClass::PairOfTens,
            4426..=4645 => HandRankClass::PairOfNines,
            4646..=4865 => HandRankClass::PairOfEights,
            4866..=5085 => HandRankClass::PairOfSevens,
            5086..=5305 => HandRankClass::PairOfSixes,
            5306..=5525 => HandRankClass::PairOfFives,
            5526..=5745 => HandRankClass::PairOfFours,
            5746..=5965 => HandRankClass::PairOfTreys,
            5966..=6185 => HandRankClass::PairOfDeuces,
            6186..=6678 => HandRankClass::AceHigh,
            6679..=7007 => HandRankClass::KingHigh,
            7008..=7216 => HandRankClass::QueenHigh,
            7217..=7341 => HandRankClass::JackHigh,
            7342..=7410 => HandRankClass::TenHigh,
            7411..=7444 => HandRankClass::NineHigh,
            7445..=7458 => HandRankClass::EightHigh,
            7459..=7462 => HandRankClass::SevenHigh,
            _ => HandRankClass::Invalid,
        }
    }

    #[must_use]
    pub fn is_a_valid_hand_rank(&self) -> bool {
        self == &HandRank::from(self.value)
    }

    #[must_use]
    pub fn is_invalid(&self) -> bool {
        self.name == HandRankName::Invalid
    }
}

impl Default for HandRank {
    fn default() -> HandRank {
        HandRank::from(0)
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<HandRankValue> for HandRank {
    fn from(value: HandRankValue) -> Self {
        HandRank {
            value,
            name: HandRank::determine_name(&value),
            class: HandRank::determine_class(&value),
        }
    }
}

impl PartialOrd<Self> for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The lower the `HandRankValue` the higher the value of the `HandRank`, unless it's invalid.
#[allow(clippy::if_same_then_else)]
impl Ord for HandRank {
    fn cmp(&self, other: &HandRank) -> Ordering {
        if self.is_invalid() && other.is_invalid() {
            Ordering::Equal
        } else if self.is_invalid() {
            Ordering::Less
        } else if other.is_invalid() {
            Ordering::Greater
        } else if self.value < other.value {
            Ordering::Greater
        } else if self.value > other.value {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

/// `HandRankValue` is the integer representing the `HandRank` for a particular five card
/// `PokerHand`. This value is used to compare one hand against the other, the lower the value,
/// the stronger the hand in a traditional, highest to lowest, ranking. A `HandRankValue` can have
/// only one `HandRankName` and `HandRankClass`.
#[allow(clippy::module_name_repetitions)]
pub type HandRankValue = u16;

pub const NO_HAND_RANK_VALUE: HandRankValue = 0;

/// `HandRankName` represents the
/// [traditional name](https://en.wikipedia.org/wiki/List_of_poker_hands) of a five card
/// `PokerHand`.
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandRankName {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
    Invalid,
}

/// `HandRankClass` represents the more specific type of the five card `PokerHand`.
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandRankClass {
    RoyalFlush,
    KingHighStraightFlush,
    QueenHighStraightFlush,
    JackHighStraightFlush,
    TenHighStraightFlush,
    NineHighStraightFlush,
    EightHighStraightFlush,
    SevenHighStraightFlush,
    SixHighStraightFlush,
    FiveHighStraightFlush,
    FourAces,
    FourKings,
    FourQueens,
    FourJacks,
    FourTens,
    FourNines,
    FourEights,
    FourSevens,
    FourSixes,
    FourFives,
    FourFours,
    FourTreys,
    FourDeuces,
    AcesOverKings,
    AcesOverQueens,
    AcesOverJacks,
    AcesOverTens,
    AcesOverNines,
    AcesOverEights,
    AcesOverSevens,
    AcesOverSixes,
    AcesOverFives,
    AcesOverFours,
    AcesOverTreys,
    AcesOverDeuces,
    KingsOverAces,
    KingsOverQueens,
    KingsOverJacks,
    KingsOverTens,
    KingsOverNines,
    KingsOverEights,
    KingsOverSevens,
    KingsOverSixes,
    KingsOverFives,
    KingsOverFours,
    KingsOverTreys,
    KingsOverDeuces,
    QueensOverAces,
    QueensOverKings,
    QueensOverJacks,
    QueensOverTens,
    QueensOverNines,
    QueensOverEights,
    QueensOverSevens,
    QueensOverSixes,
    QueensOverFives,
    QueensOverFours,
    QueensOverTreys,
    QueensOverDeuces,
    JacksOverAces,
    JacksOverKings,
    JacksOverQueens,
    JacksOverTens,
    JacksOverNines,
    JacksOverEights,
    JacksOverSevens,
    JacksOverSixes,
    JacksOverFives,
    JacksOverFours,
    JacksOverTreys,
    JacksOverDeuces,
    TensOverAces,
    TensOverKings,
    TensOverQueens,
    TensOverJacks,
    TensOverNines,
    TensOverEights,
    TensOverSevens,
    TensOverSixes,
    TensOverFives,
    TensOverFours,
    TensOverTreys,
    TensOverDeuces,
    NinesOverAces,
    NinesOverKings,
    NinesOverQueens,
    NinesOverJacks,
    NinesOverTens,
    NinesOverEights,
    NinesOverSevens,
    NinesOverSixes,
    NinesOverFives,
    NinesOverFours,
    NinesOverTreys,
    NinesOverDeuces,
    EightsOverAces,
    EightsOverKings,
    EightsOverQueens,
    EightsOverJacks,
    EightsOverTens,
    EightsOverNines,
    EightsOverSevens,
    EightsOverSixes,
    EightsOverFives,
    EightsOverFours,
    EightsOverTreys,
    EightsOverDeuces,
    SevensOverAces,
    SevensOverKings,
    SevensOverQueens,
    SevensOverJacks,
    SevensOverTens,
    SevensOverNines,
    SevensOverEights,
    SevensOverSixes,
    SevensOverFives,
    SevensOverFours,
    SevensOverTreys,
    SevensOverDeuces,
    SixesOverAces,
    SixesOverKings,
    SixesOverQueens,
    SixesOverJacks,
    SixesOverTens,
    SixesOverNines,
    SixesOverEights,
    SixesOverSevens,
    SixesOverFives,
    SixesOverFours,
    SixesOverTreys,
    SixesOverDeuces,
    FivesOverAces,
    FivesOverKings,
    FivesOverQueens,
    FivesOverJacks,
    FivesOverTens,
    FivesOverNines,
    FivesOverEights,
    FivesOverSevens,
    FivesOverSixes,
    FivesOverFours,
    FivesOverTreys,
    FivesOverDeuces,
    FoursOverAces,
    FoursOverKings,
    FoursOverQueens,
    FoursOverJacks,
    FoursOverTens,
    FoursOverNines,
    FoursOverEights,
    FoursOverSevens,
    FoursOverSixes,
    FoursOverFives,
    FoursOverTreys,
    FoursOverDeuces,
    TreysOverAces,
    TreysOverKings,
    TreysOverQueens,
    TreysOverJacks,
    TreysOverTens,
    TreysOverNines,
    TreysOverEights,
    TreysOverSevens,
    TreysOverSixes,
    TreysOverFives,
    TreysOverFours,
    TreysOverDeuces,
    DeucesOverAces,
    DeucesOverKings,
    DeucesOverQueens,
    DeucesOverJacks,
    DeucesOverTens,
    DeucesOverNines,
    DeucesOverEights,
    DeucesOverSevens,
    DeucesOverSixes,
    DeucesOverFives,
    DeucesOverFours,
    DeucesOverTreys,
    AceHighFlush,
    KingHighFlush,
    QueenHighFlush,
    JackHighFlush,
    TenHighFlush,
    NineHighFlush,
    EightHighFlush,
    SevenHighFlush,
    AceHighStraight,
    KingHighStraight,
    QueenHighStraight,
    JackHighStraight,
    TenHighStraight,
    NineHighStraight,
    EightHighStraight,
    SevenHighStraight,
    SixHighStraight,
    FiveHighStraight,
    ThreeAces,
    ThreeKings,
    ThreeQueens,
    ThreeJacks,
    ThreeTens,
    ThreeNines,
    ThreeEights,
    ThreeSevens,
    ThreeSixes,
    ThreeFives,
    ThreeFours,
    ThreeTreys,
    ThreeDeuces,
    AcesAndKings,
    AcesAndQueens,
    AcesAndJacks,
    AcesAndTens,
    AcesAndNines,
    AcesAndEights,
    AcesAndSevens,
    AcesAndSixes,
    AcesAndFives,
    AcesAndFours,
    AcesAndTreys,
    AcesAndDeuces,
    KingsAndQueens,
    KingsAndJacks,
    KingsAndTens,
    KingsAndNines,
    KingsAndEights,
    KingsAndSevens,
    KingsAndSixes,
    KingsAndFives,
    KingsAndFours,
    KingsAndTreys,
    KingsAndDeuces,
    QueensAndJacks,
    QueensAndTens,
    QueensAndNines,
    QueensAndEights,
    QueensAndSevens,
    QueensAndSixes,
    QueensAndFives,
    QueensAndFours,
    QueensAndTreys,
    QueensAndDeuces,
    JacksAndTens,
    JacksAndNines,
    JacksAndEights,
    JacksAndSevens,
    JacksAndSixes,
    JacksAndFives,
    JacksAndFours,
    JacksAndTreys,
    JacksAndDeuces,
    TensAndNines,
    TensAndEights,
    TensAndSevens,
    TensAndSixes,
    TensAndFives,
    TensAndFours,
    TensAndTreys,
    TensAndDeuces,
    NinesAndEights,
    NinesAndSevens,
    NinesAndSixes,
    NinesAndFives,
    NinesAndFours,
    NinesAndTreys,
    NinesAndDeuces,
    EightsAndSevens,
    EightsAndSixes,
    EightsAndFives,
    EightsAndFours,
    EightsAndTreys,
    EightsAndDeuces,
    SevensAndSixes,
    SevensAndFives,
    SevensAndFours,
    SevensAndTreys,
    SevensAndDeuces,
    SixesAndFives,
    SixesAndFours,
    SixesAndTreys,
    SixesAndDeuces,
    FivesAndFours,
    FivesAndTreys,
    FivesAndDeuces,
    FoursAndTreys,
    FoursAndDeuces,
    TreysAndDeuces,
    PairOfAces,
    PairOfKings,
    PairOfQueens,
    PairOfJacks,
    PairOfTens,
    PairOfNines,
    PairOfEights,
    PairOfSevens,
    PairOfSixes,
    PairOfFives,
    PairOfFours,
    PairOfTreys,
    PairOfDeuces,
    AceHigh,
    KingHigh,
    QueenHigh,
    JackHigh,
    TenHigh,
    NineHigh,
    EightHigh,
    SevenHigh,
    Invalid,
}

#[cfg(test)]
#[allow(non_snake_case)]
mod hand_rank_tests {
    use super::*;
    use crate::cards::five::Five;
    use crate::cards::HandRanker;
    use crate::parse::five_from_index;
    use alloc::format;
    use rstest::rstest;

    #[test]
    fn is_aligned() {
        assert!(HandRank::from(0).is_a_valid_hand_rank());
        assert!(HandRank::from(1).is_a_valid_hand_rank());
    }

    #[test]
    fn is_aligned__false() {
        let mut hr = HandRank::from(1);
        hr.name = HandRankName::Flush;

        assert!(!hr.is_a_valid_hand_rank());
    }

    #[test]
    fn is_invalid() {
        assert!(HandRank::from(0).is_invalid());
        assert!(HandRank::from(7463).is_invalid());
        assert!(HandRank::default().is_invalid());
        assert!(!HandRank::from(6186).is_invalid());
    }

    #[test]
    fn display() {
        assert_eq!(
            "HandRank { value: 1, name: StraightFlush, class: RoyalFlush }",
            format!("{}", HandRank::from(1))
        );
    }

    #[test]
    fn ord() {
        assert!(HandRank::from(1) > HandRank::from(2));
        assert!(HandRank::from(2000) < HandRank::from(2));
        assert!(HandRank::from(0) < HandRank::from(2));
        assert_eq!(HandRank::from(2), HandRank::from(2));
    }

    #[rustfmt::skip]
    #[rstest]
    #[case("A♠ K♠ Q♠ J♠ T♠", 1, HandRankName::StraightFlush, HandRankClass::RoyalFlush)]
    #[case("K♥ Q♥ J♥ T♥ 9♥", 2, HandRankName::StraightFlush, HandRankClass::KingHighStraightFlush)]
    #[case("Q♦ J♦ T♦ 9♦ 8♦", 3, HandRankName::StraightFlush, HandRankClass::QueenHighStraightFlush)]
    #[case("J♣ T♣ 9♣ 8♣ 7♣", 4, HandRankName::StraightFlush, HandRankClass::JackHighStraightFlush)]
    #[case("T♤ 9♤ 8♤ 7♤ 6♤", 5, HandRankName::StraightFlush, HandRankClass::TenHighStraightFlush)]
    #[case("9♡ 8♡ 7♡ 6♡ 5♡", 6, HandRankName::StraightFlush, HandRankClass::NineHighStraightFlush)]
    #[case("8♧ 7♧ 6♧ 5♧ 4♧", 7, HandRankName::StraightFlush, HandRankClass::EightHighStraightFlush)]
    #[case("7S 6S  5S 4S 3S", 8, HandRankName::StraightFlush, HandRankClass::SevenHighStraightFlush)]
    #[case("6H 5H 4H 3H 2H", 9, HandRankName::StraightFlush, HandRankClass::SixHighStraightFlush)]
    #[case("5D 4D 3D 2D AD", 10, HandRankName::StraightFlush, HandRankClass::FiveHighStraightFlush)]
    #[case("AS AH AD AC KS", 11, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("AS AH AD AC 2S", 22, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("KS KH KD KC AS", 23, HandRankName::FourOfAKind, HandRankClass::FourKings)]
    #[case("KS KH KD KC 2S", 34, HandRankName::FourOfAKind, HandRankClass::FourKings)]
    #[case("QS QH QD QC AS", 35, HandRankName::FourOfAKind, HandRankClass::FourQueens)]
    #[case("QS QH QD QC 2C", 46, HandRankName::FourOfAKind, HandRankClass::FourQueens)]
    #[case("JS JH JD JC AC", 47, HandRankName::FourOfAKind, HandRankClass::FourJacks)]
    #[case("JS JH JD JC 2C", 58, HandRankName::FourOfAKind, HandRankClass::FourJacks)]
    #[case("TS TH TD TC AS", 59, HandRankName::FourOfAKind, HandRankClass::FourTens)]
    #[case("TS TH TD TC 2C", 70, HandRankName::FourOfAKind, HandRankClass::FourTens)]
    #[case("9S 9H 9D 9C AH", 71, HandRankName::FourOfAKind, HandRankClass::FourNines)]
    #[case("9S 9H 9D 9C 2D", 82, HandRankName::FourOfAKind, HandRankClass::FourNines)]
    #[case("8S 8H 8D 8C AD", 83, HandRankName::FourOfAKind, HandRankClass::FourEights)]
    #[case("8S 8H 8D 8C 2D", 94, HandRankName::FourOfAKind, HandRankClass::FourEights)]
    #[case("7S 7H 7D 7C AD", 95, HandRankName::FourOfAKind, HandRankClass::FourSevens)]
    #[case("7S 7H 7D 7C 2D", 106, HandRankName::FourOfAKind, HandRankClass::FourSevens)]
    #[case("6S 6H 6D 6C AD", 107, HandRankName::FourOfAKind, HandRankClass::FourSixes)]
    #[case("6S 6H 6D 6C 2D", 118, HandRankName::FourOfAKind, HandRankClass::FourSixes)]
    #[case("5S 5H 5D 5C AD", 119, HandRankName::FourOfAKind, HandRankClass::FourFives)]
    #[case("5S 5H 5D 5C 2D", 130, HandRankName::FourOfAKind, HandRankClass::FourFives)]
    #[case("4S 4H 4D 4C AD", 131, HandRankName::FourOfAKind, HandRankClass::FourFours)]
    #[case("4S 4H 4D 4C 2D", 142, HandRankName::FourOfAKind, HandRankClass::FourFours)]
    #[case("3S 3H 3D 3C AD", 143, HandRankName::FourOfAKind, HandRankClass::FourTreys)]
    #[case("3S 3H 3D 3C 2D", 154, HandRankName::FourOfAKind, HandRankClass::FourTreys)]
    #[case("2S 2H 2D 2C AC", 155, HandRankName::FourOfAKind, HandRankClass::FourDeuces)]
    #[case("2S 2H 2D 2C 3D", 166, HandRankName::FourOfAKind, HandRankClass::FourDeuces)]
    #[case("AS AH AD KC KD", 167, HandRankName::FullHouse, HandRankClass::AcesOverKings)]
    #[case("AS AH AD QC QD", 168, HandRankName::FullHouse, HandRankClass::AcesOverQueens)]
    #[case("AS AH AD JD JC", 169, HandRankName::FullHouse, HandRankClass::AcesOverJacks)]
    #[case("AS AH AD TD TC", 170, HandRankName::FullHouse, HandRankClass::AcesOverTens)]
    #[case("AS AH AD 9S 9D", 171, HandRankName::FullHouse, HandRankClass::AcesOverNines)]
    #[case("AS AH AD 8S 8D", 172, HandRankName::FullHouse, HandRankClass::AcesOverEights)]
    #[case("AS AH AD 7S 7D", 173, HandRankName::FullHouse, HandRankClass::AcesOverSevens)]
    #[case("AS AH AD 6S 6D", 174, HandRankName::FullHouse, HandRankClass::AcesOverSixes)]
    #[case("AS AH AD 5S 5D", 175, HandRankName::FullHouse, HandRankClass::AcesOverFives)]
    #[case("AS AH AD 4S 4D", 176, HandRankName::FullHouse, HandRankClass::AcesOverFours)]
    #[case("AS AH AD 3D 3c", 177, HandRankName::FullHouse, HandRankClass::AcesOverTreys)]
    #[case("AS AH AD 2H 2D", 178, HandRankName::FullHouse, HandRankClass::AcesOverDeuces)]
    #[case("AS AH KD KH KC", 179, HandRankName::FullHouse, HandRankClass::KingsOverAces)]
    #[case("QS KH QD KC KD", 180, HandRankName::FullHouse, HandRankClass::KingsOverQueens)]
    #[case("KS KH KD JH JD", 181, HandRankName::FullHouse, HandRankClass::KingsOverJacks)]
    #[case("KS KH KD TH TD", 182, HandRankName::FullHouse, HandRankClass::KingsOverTens)]
    #[case("KS KH KD 9H 9D", 183, HandRankName::FullHouse, HandRankClass::KingsOverNines)]
    #[case("KS KH 8D 8H KD", 184, HandRankName::FullHouse, HandRankClass::KingsOverEights)]
    #[case("KS KH KD 7H 7D", 185, HandRankName::FullHouse, HandRankClass::KingsOverSevens)]
    #[case("KS KH KD 6H 6D", 186, HandRankName::FullHouse, HandRankClass::KingsOverSixes)]
    #[case("KS KH KD 5H 5D", 187, HandRankName::FullHouse, HandRankClass::KingsOverFives)]
    #[case("4S 4H KD KH KC", 188, HandRankName::FullHouse, HandRankClass::KingsOverFours)]
    #[case("3S KH KD 3H KC", 189, HandRankName::FullHouse, HandRankClass::KingsOverTreys)]
    #[case("KS KH KD 2H 2D", 190, HandRankName::FullHouse, HandRankClass::KingsOverDeuces)]
    #[case("QS QH QD AH AD", 191, HandRankName::FullHouse, HandRankClass::QueensOverAces)]
    #[case("QS QH QD KH KD", 192, HandRankName::FullHouse, HandRankClass::QueensOverKings)]
    #[case("QS QH QD JH JD", 193, HandRankName::FullHouse, HandRankClass::QueensOverJacks)]
    #[case("QS QH QD TH TD", 194, HandRankName::FullHouse, HandRankClass::QueensOverTens)]
    #[case("QS QH QD 9H 9D", 195, HandRankName::FullHouse, HandRankClass::QueensOverNines)]
    #[case("QS QH QD 8H 8D", 196, HandRankName::FullHouse, HandRankClass::QueensOverEights)]
    #[case("QS QH QD 7H 7D", 197, HandRankName::FullHouse, HandRankClass::QueensOverSevens)]
    #[case("QS QH QD 6H 6D", 198, HandRankName::FullHouse, HandRankClass::QueensOverSixes)]
    #[case("QS QH QD 5H 5D", 199, HandRankName::FullHouse, HandRankClass::QueensOverFives)]
    #[case("QS QH QD 4S 4D", 200, HandRankName::FullHouse, HandRankClass::QueensOverFours)]
    #[case("QS QH QD 3H 3D", 201, HandRankName::FullHouse, HandRankClass::QueensOverTreys)]
    #[case("QS QH QD 2H 2D", 202, HandRankName::FullHouse, HandRankClass::QueensOverDeuces)]
    #[case("JS JH JD AH AD", 203, HandRankName::FullHouse, HandRankClass::JacksOverAces)]
    #[case("JS JH JD KH KD", 204, HandRankName::FullHouse, HandRankClass::JacksOverKings)]
    #[case("JS JH JD QH QD", 205, HandRankName::FullHouse, HandRankClass::JacksOverQueens)]
    #[case("JS JH JD TH TD", 206, HandRankName::FullHouse, HandRankClass::JacksOverTens)]
    #[case("JS JH JD 9H 9D", 207, HandRankName::FullHouse, HandRankClass::JacksOverNines)]
    #[case("JS JH JD 8H 8D", 208, HandRankName::FullHouse, HandRankClass::JacksOverEights)]
    #[case("JS JH JD 7H 7D", 209, HandRankName::FullHouse, HandRankClass::JacksOverSevens)]
    #[case("JS JH JD 6H 6D", 210, HandRankName::FullHouse, HandRankClass::JacksOverSixes)]
    #[case("JS JH JD 5H 5D", 211, HandRankName::FullHouse, HandRankClass::JacksOverFives)]
    #[case("JS JH JD 4H 4D", 212, HandRankName::FullHouse, HandRankClass::JacksOverFours)]
    #[case("JS JH JD 3H 3D", 213, HandRankName::FullHouse, HandRankClass::JacksOverTreys)]
    #[case("JS JH JD 2H 2D", 214, HandRankName::FullHouse, HandRankClass::JacksOverDeuces)]
    #[case("TS TH TD AH AD", 215, HandRankName::FullHouse, HandRankClass::TensOverAces)]
    #[case("TS TH TD KH KD", 216, HandRankName::FullHouse, HandRankClass::TensOverKings)]
    #[case("TS TH TD QH QD", 217, HandRankName::FullHouse, HandRankClass::TensOverQueens)]
    #[case("TS TH TD JH JD", 218, HandRankName::FullHouse, HandRankClass::TensOverJacks)]
    #[case("TS TH TD 9H 9D", 219, HandRankName::FullHouse, HandRankClass::TensOverNines)]
    #[case("TS TH TD 8H 8D", 220, HandRankName::FullHouse, HandRankClass::TensOverEights)]
    #[case("TS TH TD 7H 7D", 221, HandRankName::FullHouse, HandRankClass::TensOverSevens)]
    #[case("TS TH TD 6S 6D", 222, HandRankName::FullHouse, HandRankClass::TensOverSixes)]
    #[case("TS TH TD 5H 5D", 223, HandRankName::FullHouse, HandRankClass::TensOverFives)]
    #[case("TS TH TD 4H 4D", 224, HandRankName::FullHouse, HandRankClass::TensOverFours)]
    #[case("TS TH TD 3H 3D", 225, HandRankName::FullHouse, HandRankClass::TensOverTreys)]
    #[case("TS TH TD 2H 2D", 226, HandRankName::FullHouse, HandRankClass::TensOverDeuces)]
    #[case("9S 9H 9D AH AD", 227, HandRankName::FullHouse, HandRankClass::NinesOverAces)]
    #[case("9S 9H 9D KH KD", 228, HandRankName::FullHouse, HandRankClass::NinesOverKings)]
    #[case("9S 9H 9D QH QD", 229, HandRankName::FullHouse, HandRankClass::NinesOverQueens)]
    #[case("9S 9H 9D JH JD", 230, HandRankName::FullHouse, HandRankClass::NinesOverJacks)]
    #[case("9S 9H 9D TH TD", 231, HandRankName::FullHouse, HandRankClass::NinesOverTens)]
    #[case("9S 9H 9D 8H 8D", 232, HandRankName::FullHouse, HandRankClass::NinesOverEights)]
    #[case("9S 9H 9D 7H 7D", 233, HandRankName::FullHouse, HandRankClass::NinesOverSevens)]
    #[case("9S 9H 9D 6S 6D", 234, HandRankName::FullHouse, HandRankClass::NinesOverSixes)]
    #[case("9S 9H 9D 5H 5D", 235, HandRankName::FullHouse, HandRankClass::NinesOverFives)]
    #[case("9S 9H 9D 4H 4D", 236, HandRankName::FullHouse, HandRankClass::NinesOverFours)]
    #[case("9S 9H 9D 3H 3D", 237, HandRankName::FullHouse, HandRankClass::NinesOverTreys)]
    #[case("9S 9H 9D 2H 2D", 238, HandRankName::FullHouse, HandRankClass::NinesOverDeuces)]
    #[case("8S 8H 8D AH AD", 239, HandRankName::FullHouse, HandRankClass::EightsOverAces)]
    #[case("8S 8H 8D KH KD", 240, HandRankName::FullHouse, HandRankClass::EightsOverKings)]
    #[case("8S 8H 8D QH QD", 241, HandRankName::FullHouse, HandRankClass::EightsOverQueens)]
    #[case("8S 8H 8D JH JD", 242, HandRankName::FullHouse, HandRankClass::EightsOverJacks)]
    #[case("8S 8H 8D TH TD", 243, HandRankName::FullHouse, HandRankClass::EightsOverTens)]
    #[case("8S 8H 8D 9H 9D", 244, HandRankName::FullHouse, HandRankClass::EightsOverNines)]
    #[case("8S 8H 8D 7H 7D", 245, HandRankName::FullHouse, HandRankClass::EightsOverSevens)]
    #[case("8S 8H 8D 6S 6D", 246, HandRankName::FullHouse, HandRankClass::EightsOverSixes)]
    #[case("8S 8H 8D 5H 5D", 247, HandRankName::FullHouse, HandRankClass::EightsOverFives)]
    #[case("8S 8H 8D 4H 4D", 248, HandRankName::FullHouse, HandRankClass::EightsOverFours)]
    #[case("8S 8H 8D 3H 3D", 249, HandRankName::FullHouse, HandRankClass::EightsOverTreys)]
    #[case("8S 8H 8D 2H 2D", 250, HandRankName::FullHouse, HandRankClass::EightsOverDeuces)]
    #[case("7S 7H 7D AH AD", 251, HandRankName::FullHouse, HandRankClass::SevensOverAces)]
    #[case("7S 7H 7D KH KD", 252, HandRankName::FullHouse, HandRankClass::SevensOverKings)]
    #[case("7S 7H 7D QH QD", 253, HandRankName::FullHouse, HandRankClass::SevensOverQueens)]
    #[case("7S 7H 7D JH JD", 254, HandRankName::FullHouse, HandRankClass::SevensOverJacks)]
    #[case("7S 7H 7D TH TD", 255, HandRankName::FullHouse, HandRankClass::SevensOverTens)]
    #[case("7S 7H 7D 9H 9D", 256, HandRankName::FullHouse, HandRankClass::SevensOverNines)]
    #[case("7S 7H 7D 8H 8D", 257, HandRankName::FullHouse, HandRankClass::SevensOverEights)]
    #[case("7S 7H 7D 6S 6D", 258, HandRankName::FullHouse, HandRankClass::SevensOverSixes)]
    #[case("7S 7H 7D 5H 5D", 259, HandRankName::FullHouse, HandRankClass::SevensOverFives)]
    #[case("7S 7H 7D 4H 4D", 260, HandRankName::FullHouse, HandRankClass::SevensOverFours)]
    #[case("7S 7H 7D 3H 3D", 261, HandRankName::FullHouse, HandRankClass::SevensOverTreys)]
    #[case("7S 7H 7D 2H 2D", 262, HandRankName::FullHouse, HandRankClass::SevensOverDeuces)]
    #[case("6S 6H 6D AH AD", 263, HandRankName::FullHouse, HandRankClass::SixesOverAces)]
    #[case("6S 6H 6D KH KD", 264, HandRankName::FullHouse, HandRankClass::SixesOverKings)]
    #[case("6S 6H 6D QH QD", 265, HandRankName::FullHouse, HandRankClass::SixesOverQueens)]
    #[case("6S 6H 6D JH JD", 266, HandRankName::FullHouse, HandRankClass::SixesOverJacks)]
    #[case("6S 6H 6D TH TD", 267, HandRankName::FullHouse, HandRankClass::SixesOverTens)]
    #[case("6S 6H 6D 9H 9D", 268, HandRankName::FullHouse, HandRankClass::SixesOverNines)]
    #[case("6S 6H 6D 8H 8D", 269, HandRankName::FullHouse, HandRankClass::SixesOverEights)]
    #[case("6S 6H 6D 7S 7D", 270, HandRankName::FullHouse, HandRankClass::SixesOverSevens)]
    #[case("6S 6H 6D 5H 5D", 271, HandRankName::FullHouse, HandRankClass::SixesOverFives)]
    #[case("6S 6H 6D 4H 4D", 272, HandRankName::FullHouse, HandRankClass::SixesOverFours)]
    #[case("6S 6H 6D 3H 3D", 273, HandRankName::FullHouse, HandRankClass::SixesOverTreys)]
    #[case("6S 6H 6D 2H 2D", 274, HandRankName::FullHouse, HandRankClass::SixesOverDeuces)]
    #[case("5S 5H 5D AH AD", 275, HandRankName::FullHouse, HandRankClass::FivesOverAces)]
    #[case("5S 5H 5D KH KD", 276, HandRankName::FullHouse, HandRankClass::FivesOverKings)]
    #[case("5S 5H 5D QH QD", 277, HandRankName::FullHouse, HandRankClass::FivesOverQueens)]
    #[case("5S 5H 5D JH JD", 278, HandRankName::FullHouse, HandRankClass::FivesOverJacks)]
    #[case("5S 5H 5D TH TD", 279, HandRankName::FullHouse, HandRankClass::FivesOverTens)]
    #[case("5S 5H 5D 9H 9D", 280, HandRankName::FullHouse, HandRankClass::FivesOverNines)]
    #[case("5S 5H 5D 8H 8D", 281, HandRankName::FullHouse, HandRankClass::FivesOverEights)]
    #[case("5S 5H 5D 7S 7D", 282, HandRankName::FullHouse, HandRankClass::FivesOverSevens)]
    #[case("5S 5H 5D 6H 6D", 283, HandRankName::FullHouse, HandRankClass::FivesOverSixes)]
    #[case("5S 5H 5D 4H 4D", 284, HandRankName::FullHouse, HandRankClass::FivesOverFours)]
    #[case("5S 5H 5D 3H 3D", 285, HandRankName::FullHouse, HandRankClass::FivesOverTreys)]
    #[case("5S 5H 5D 2H 2D", 286, HandRankName::FullHouse, HandRankClass::FivesOverDeuces)]
    #[case("4S 4H 4D AH AD", 287, HandRankName::FullHouse, HandRankClass::FoursOverAces)]
    #[case("4S 4H 4D KH KD", 288, HandRankName::FullHouse, HandRankClass::FoursOverKings)]
    #[case("4S 4H 4D QH QD", 289, HandRankName::FullHouse, HandRankClass::FoursOverQueens)]
    #[case("4S 4H 4D JH JD", 290, HandRankName::FullHouse, HandRankClass::FoursOverJacks)]
    #[case("4S 4H 4D TH TD", 291, HandRankName::FullHouse, HandRankClass::FoursOverTens)]
    #[case("4S 4H 4D 9H 9D", 292, HandRankName::FullHouse, HandRankClass::FoursOverNines)]
    #[case("4S 4H 4D 8H 8D", 293, HandRankName::FullHouse, HandRankClass::FoursOverEights)]
    #[case("4S 4H 4D 7S 7D", 294, HandRankName::FullHouse, HandRankClass::FoursOverSevens)]
    #[case("4S 4H 4D 6H 6D", 295, HandRankName::FullHouse, HandRankClass::FoursOverSixes)]
    #[case("4S 4H 4D 5H 5D", 296, HandRankName::FullHouse, HandRankClass::FoursOverFives)]
    #[case("4S 4H 4D 3H 3D", 297, HandRankName::FullHouse, HandRankClass::FoursOverTreys)]
    #[case("4S 4H 4D 2H 2D", 298, HandRankName::FullHouse, HandRankClass::FoursOverDeuces)]
    #[case("3S 3H 3D AH AD", 299, HandRankName::FullHouse, HandRankClass::TreysOverAces)]
    #[case("3S 3H 3D KH KD", 300, HandRankName::FullHouse, HandRankClass::TreysOverKings)]
    #[case("3S 3H 3D QH QD", 301, HandRankName::FullHouse, HandRankClass::TreysOverQueens)]
    #[case("3S 3H 3D JH JD", 302, HandRankName::FullHouse, HandRankClass::TreysOverJacks)]
    #[case("3S 3H 3D TH TD", 303, HandRankName::FullHouse, HandRankClass::TreysOverTens)]
    #[case("3S 3H 3D 9H 9D", 304, HandRankName::FullHouse, HandRankClass::TreysOverNines)]
    #[case("3S 3H 3D 8H 8D", 305, HandRankName::FullHouse, HandRankClass::TreysOverEights)]
    #[case("3S 3H 3D 7S 7D", 306, HandRankName::FullHouse, HandRankClass::TreysOverSevens)]
    #[case("3S 3H 3D 6H 6D", 307, HandRankName::FullHouse, HandRankClass::TreysOverSixes)]
    #[case("3S 3H 3D 5H 5D", 308, HandRankName::FullHouse, HandRankClass::TreysOverFives)]
    #[case("3S 3H 3D 4H 4D", 309, HandRankName::FullHouse, HandRankClass::TreysOverFours)]
    #[case("3S 3H 3D 2H 2D", 310, HandRankName::FullHouse, HandRankClass::TreysOverDeuces)]
    #[case("2S 2H 2D AH AD", 311, HandRankName::FullHouse, HandRankClass::DeucesOverAces)]
    #[case("2S 2H 2D KH KD", 312, HandRankName::FullHouse, HandRankClass::DeucesOverKings)]
    #[case("2S 2H 2D QH QD", 313, HandRankName::FullHouse, HandRankClass::DeucesOverQueens)]
    #[case("2S 2H 2D JH JD", 314, HandRankName::FullHouse, HandRankClass::DeucesOverJacks)]
    #[case("2S 2H 2D TH TD", 315, HandRankName::FullHouse, HandRankClass::DeucesOverTens)]
    #[case("2S 2H 2D 9H 9D", 316, HandRankName::FullHouse, HandRankClass::DeucesOverNines)]
    #[case("2S 2H 2D 8H 8D", 317, HandRankName::FullHouse, HandRankClass::DeucesOverEights)]
    #[case("2S 2H 2D 7S 7D", 318, HandRankName::FullHouse, HandRankClass::DeucesOverSevens)]
    #[case("2S 2H 2D 6H 6D", 319, HandRankName::FullHouse, HandRankClass::DeucesOverSixes)]
    #[case("2S 2H 2D 5H 5D", 320, HandRankName::FullHouse, HandRankClass::DeucesOverFives)]
    #[case("2S 2H 2D 4H 4D", 321, HandRankName::FullHouse, HandRankClass::DeucesOverFours)]
    #[case("2S 2H 2D 3H 3D", 322, HandRankName::FullHouse, HandRankClass::DeucesOverTreys)]
    #[case("AS KS QS JS 9S", 323, HandRankName::Flush, HandRankClass::AceHighFlush)]
    #[case("AS 6S 4S 3S 2S", 815, HandRankName::Flush, HandRankClass::AceHighFlush)]
    #[case("KH QH JH TH 8H", 816, HandRankName::Flush, HandRankClass::KingHighFlush)]
    #[case("KC 5C 4C 3C 2C", 1144, HandRankName::Flush, HandRankClass::KingHighFlush)]
    #[case("QH JH TH 9H 7H", 1145, HandRankName::Flush, HandRankClass::QueenHighFlush)]
    #[case("QC 5C 4C 3C 2C", 1353, HandRankName::Flush, HandRankClass::QueenHighFlush)]
    #[case("JH TH 9H 8H 6H", 1354, HandRankName::Flush, HandRankClass::JackHighFlush)]
    #[case("JC 5C 4C 3C 2C", 1478, HandRankName::Flush, HandRankClass::JackHighFlush)]
    #[case("TH 9H 8H 7H 5H", 1479, HandRankName::Flush, HandRankClass::TenHighFlush)]
    #[case("TC 5C 4C 3C 2C", 1547, HandRankName::Flush, HandRankClass::TenHighFlush)]
    #[case("9H 8H 7H 6H 4H", 1548, HandRankName::Flush, HandRankClass::NineHighFlush)]
    #[case("9C 5C 4C 3C 2C", 1581, HandRankName::Flush, HandRankClass::NineHighFlush)]
    #[case("8H 7H 6H 5H 3H", 1582, HandRankName::Flush, HandRankClass::EightHighFlush)]
    #[case("8C 5C 4C 3C 2C", 1595, HandRankName::Flush, HandRankClass::EightHighFlush)]
    #[case("7H 6H 5H 4H 2H", 1596, HandRankName::Flush, HandRankClass::SevenHighFlush)]
    #[case("7C 5C 4C 3C 2C", 1599, HandRankName::Flush, HandRankClass::SevenHighFlush)]
    #[case("A♠ K♠ Q♥ J♠ T♠", 1600, HandRankName::Straight, HandRankClass::AceHighStraight)]
    #[case("K♥ Q♥ J♠ T♥ 9♥", 1601, HandRankName::Straight, HandRankClass::KingHighStraight)]
    #[case("Q♦ J♠ T♦ 9♦ 8♦", 1602, HandRankName::Straight, HandRankClass::QueenHighStraight)]
    #[case("J♣ T♣ 9♣ 8♠ 7♣", 1603, HandRankName::Straight, HandRankClass::JackHighStraight)]
    #[case("T♤ 9♤ 8♡ 7♤ 6♤", 1604, HandRankName::Straight, HandRankClass::TenHighStraight)]
    #[case("9♡ 8♤ 7♡ 6♡ 5♡", 1605, HandRankName::Straight, HandRankClass::NineHighStraight)]
    #[case("8♧ 7♧ 6♡ 5♧ 4♧", 1606, HandRankName::Straight, HandRankClass::EightHighStraight)]
    #[case("7S 6♥ 5S 4S 3S", 1607, HandRankName::Straight, HandRankClass::SevenHighStraight)]
    #[case("6H 5S 4H 3H 2H", 1608, HandRankName::Straight, HandRankClass::SixHighStraight)]
    #[case("5D 4D 3♥ 2D AD", 1609, HandRankName::Straight, HandRankClass::FiveHighStraight)]
    #[case("AS AD AC KS QD", 1610, HandRankName::ThreeOfAKind, HandRankClass::ThreeAces)]
    #[case("AS AD AC 3S 2D", 1675, HandRankName::ThreeOfAKind, HandRankClass::ThreeAces)]
    #[case("KS KH KC AD QD", 1676, HandRankName::ThreeOfAKind, HandRankClass::ThreeKings)]
    #[case("KS KH KC 3D 2D", 1741, HandRankName::ThreeOfAKind, HandRankClass::ThreeKings)]
    #[case("QH QD QC AD KS", 1742, HandRankName::ThreeOfAKind, HandRankClass::ThreeQueens)]
    #[case("QH QD QC 3D 2S", 1807, HandRankName::ThreeOfAKind, HandRankClass::ThreeQueens)]
    #[case("JS JD JC AD KS", 1808, HandRankName::ThreeOfAKind, HandRankClass::ThreeJacks)]
    #[case("JS JD JC 3D 2S", 1873, HandRankName::ThreeOfAKind, HandRankClass::ThreeJacks)]
    #[case("TH TD TC AD KD", 1874, HandRankName::ThreeOfAKind, HandRankClass::ThreeTens)]
    #[case("TH TD TC 3D 2D", 1939, HandRankName::ThreeOfAKind, HandRankClass::ThreeTens)]
    #[case("9H 9D 9C AD KD", 1940, HandRankName::ThreeOfAKind, HandRankClass::ThreeNines)]
    #[case("9H 9D 9C 3D 2D", 2005, HandRankName::ThreeOfAKind, HandRankClass::ThreeNines)]
    #[case("8H 8D 8C AD KD", 2006, HandRankName::ThreeOfAKind, HandRankClass::ThreeEights)]
    #[case("8H 8D 8C 3D 2D", 2071, HandRankName::ThreeOfAKind, HandRankClass::ThreeEights)]
    #[case("7H 7D 7C AS KD", 2072, HandRankName::ThreeOfAKind, HandRankClass::ThreeSevens)]
    #[case("7H 7D 7C 3S 2D", 2137, HandRankName::ThreeOfAKind, HandRankClass::ThreeSevens)]
    #[case("6H 6D 6C AS KD", 2138, HandRankName::ThreeOfAKind, HandRankClass::ThreeSixes)]
    #[case("6H 6D 6C 3S 2D", 2203, HandRankName::ThreeOfAKind, HandRankClass::ThreeSixes)]
    #[case("5S 5H 5C AD KD", 2204, HandRankName::ThreeOfAKind, HandRankClass::ThreeFives)]
    #[case("5S 5H 5C 3D 2D", 2269, HandRankName::ThreeOfAKind, HandRankClass::ThreeFives)]
    #[case("4S 4H 4C AD KD", 2270, HandRankName::ThreeOfAKind, HandRankClass::ThreeFours)]
    #[case("4S 4H 4C 3D 2D", 2335, HandRankName::ThreeOfAKind, HandRankClass::ThreeFours)]
    #[case("3S 3H 3C AD KD", 2336, HandRankName::ThreeOfAKind, HandRankClass::ThreeTreys)]
    #[case("3S 3D 3C 4D 2D", 2401, HandRankName::ThreeOfAKind, HandRankClass::ThreeTreys)]
    #[case("2S 2H 2C AD KD", 2402, HandRankName::ThreeOfAKind, HandRankClass::ThreeDeuces)]
    #[case("2S 2H 2C 4S 3C", 2467, HandRankName::ThreeOfAKind, HandRankClass::ThreeDeuces)]
    #[case("AS AD KS KH Q♥", 2468, HandRankName::TwoPair, HandRankClass::AcesAndKings)]
    #[case("AS AD KS KH 2♥", 2478, HandRankName::TwoPair, HandRankClass::AcesAndKings)]
    #[case("AS AD QS QH K♥", 2479, HandRankName::TwoPair, HandRankClass::AcesAndQueens)]
    #[case("AS AD QS QH 2♥", 2489, HandRankName::TwoPair, HandRankClass::AcesAndQueens)]
    #[case("AS AD JS JH K♥", 2490, HandRankName::TwoPair, HandRankClass::AcesAndJacks)]
    #[case("AS AD JS JH 2♥", 2500, HandRankName::TwoPair, HandRankClass::AcesAndJacks)]
    #[case("AS AD TS TH K♥", 2501, HandRankName::TwoPair, HandRankClass::AcesAndTens)]
    #[case("AS AD TS TH 2♥", 2511, HandRankName::TwoPair, HandRankClass::AcesAndTens)]
    #[case("AS AD 9S 9H K♥", 2512, HandRankName::TwoPair, HandRankClass::AcesAndNines)]
    #[case("AS AD 9S 9H 2♥", 2522, HandRankName::TwoPair, HandRankClass::AcesAndNines)]
    #[case("AS AD 8S 8H K♥", 2523, HandRankName::TwoPair, HandRankClass::AcesAndEights)]
    #[case("AS AD 8S 8H 2♥", 2533, HandRankName::TwoPair, HandRankClass::AcesAndEights)]
    #[case("AS AD 7S 7H K♥", 2534, HandRankName::TwoPair, HandRankClass::AcesAndSevens)]
    #[case("AS AD 7S 7H 2♥", 2544, HandRankName::TwoPair, HandRankClass::AcesAndSevens)]
    #[case("AS AD 6S 6H K♥", 2545, HandRankName::TwoPair, HandRankClass::AcesAndSixes)]
    #[case("AS AD 6S 6H 2♥", 2555, HandRankName::TwoPair, HandRankClass::AcesAndSixes)]
    #[case("AS AD 5S 5H K♥", 2556, HandRankName::TwoPair, HandRankClass::AcesAndFives)]
    #[case("AS AD 5S 5H 2♥", 2566, HandRankName::TwoPair, HandRankClass::AcesAndFives)]
    #[case("AS AD 4S 4H K♥", 2567, HandRankName::TwoPair, HandRankClass::AcesAndFours)]
    #[case("AS AD 4S 4H 2♥", 2577, HandRankName::TwoPair, HandRankClass::AcesAndFours)]
    #[case("AS AD 3S 3H K♥", 2578, HandRankName::TwoPair, HandRankClass::AcesAndTreys)]
    #[case("AS AD 3S 3H 2♥", 2588, HandRankName::TwoPair, HandRankClass::AcesAndTreys)]
    #[case("AS AD 2S 2H K♥", 2589, HandRankName::TwoPair, HandRankClass::AcesAndDeuces)]
    #[case("AS AD 2S 2H 3♥", 2599, HandRankName::TwoPair, HandRankClass::AcesAndDeuces)]
    #[case("KS KH Q♥ QD AC", 2600, HandRankName::TwoPair, HandRankClass::KingsAndQueens)]
    #[case("KS KH Q♥ QD 2♥", 2610, HandRankName::TwoPair, HandRankClass::KingsAndQueens)]
    #[case("KS KH J♥ JD AC", 2611, HandRankName::TwoPair, HandRankClass::KingsAndJacks)]
    #[case("KS KH J♥ JD 2♥", 2621, HandRankName::TwoPair, HandRankClass::KingsAndJacks)]
    #[case("KS KH T♥ TD AC", 2622, HandRankName::TwoPair, HandRankClass::KingsAndTens)]
    #[case("KS KH T♥ TD 2♥", 2632, HandRankName::TwoPair, HandRankClass::KingsAndTens)]
    #[case("KS KH 9♥ 9D AC", 2633, HandRankName::TwoPair, HandRankClass::KingsAndNines)]
    #[case("KS KH 9♥ 9D 2♥", 2643, HandRankName::TwoPair, HandRankClass::KingsAndNines)]
    #[case("KS KH 8♥ 8D AC", 2644, HandRankName::TwoPair, HandRankClass::KingsAndEights)]
    #[case("KS KH 8♥ 8D 2♥", 2654, HandRankName::TwoPair, HandRankClass::KingsAndEights)]
    #[case("KS KH 7♥ 7D AC", 2655, HandRankName::TwoPair, HandRankClass::KingsAndSevens)]
    #[case("KS KH 7♥ 7D 2♥", 2665, HandRankName::TwoPair, HandRankClass::KingsAndSevens)]
    #[case("KS KH 6♥ 6D AC", 2666, HandRankName::TwoPair, HandRankClass::KingsAndSixes)]
    #[case("KS KH 6♥ 6D 2♥", 2676, HandRankName::TwoPair, HandRankClass::KingsAndSixes)]
    #[case("KS KH 5♥ 5D AC", 2677, HandRankName::TwoPair, HandRankClass::KingsAndFives)]
    #[case("KS KH 5♥ 5D 2♥", 2687, HandRankName::TwoPair, HandRankClass::KingsAndFives)]
    #[case("KS KH 4♥ 4D AC", 2688, HandRankName::TwoPair, HandRankClass::KingsAndFours)]
    #[case("KS KH 4♥ 4D 2♥", 2698, HandRankName::TwoPair, HandRankClass::KingsAndFours)]
    #[case("KS KH 3♥ 3D AC", 2699, HandRankName::TwoPair, HandRankClass::KingsAndTreys)]
    #[case("KS KH 3♥ 3D 2♥", 2709, HandRankName::TwoPair, HandRankClass::KingsAndTreys)]
    #[case("KS KH 2♥ 2D AC", 2710, HandRankName::TwoPair, HandRankClass::KingsAndDeuces)]
    #[case("KS KH 2♥ 2D 3♥", 2720, HandRankName::TwoPair, HandRankClass::KingsAndDeuces)]
    #[case("QS QH J♥ JD AC", 2721, HandRankName::TwoPair, HandRankClass::QueensAndJacks)]
    #[case("QS QH J♥ JD 2♥", 2731, HandRankName::TwoPair, HandRankClass::QueensAndJacks)]
    #[case("QS QH T♥ TD AC", 2732, HandRankName::TwoPair, HandRankClass::QueensAndTens)]
    #[case("QS QH T♥ TD 2♥", 2742, HandRankName::TwoPair, HandRankClass::QueensAndTens)]
    #[case("QS QH 9♥ 9D AC", 2743, HandRankName::TwoPair, HandRankClass::QueensAndNines)]
    #[case("QS QH 9♥ 9D 2♥", 2753, HandRankName::TwoPair, HandRankClass::QueensAndNines)]
    #[case("QS QH 8♥ 8D AC", 2754, HandRankName::TwoPair, HandRankClass::QueensAndEights)]
    #[case("QS QH 8♥ 8D 2♥", 2764, HandRankName::TwoPair, HandRankClass::QueensAndEights)]
    #[case("QS QH 7♥ 7D AC", 2765, HandRankName::TwoPair, HandRankClass::QueensAndSevens)]
    #[case("QS QH 7♥ 7D 2♥", 2775, HandRankName::TwoPair, HandRankClass::QueensAndSevens)]
    #[case("QS QH 6♥ 6D AC", 2776, HandRankName::TwoPair, HandRankClass::QueensAndSixes)]
    #[case("QS QH 6♥ 6D 2♥", 2786, HandRankName::TwoPair, HandRankClass::QueensAndSixes)]
    #[case("QS QH 5♥ 5D AC", 2787, HandRankName::TwoPair, HandRankClass::QueensAndFives)]
    #[case("QS QH 5♥ 5D 2♥", 2797, HandRankName::TwoPair, HandRankClass::QueensAndFives)]
    #[case("QS QH 4♥ 4D AC", 2798, HandRankName::TwoPair, HandRankClass::QueensAndFours)]
    #[case("QS QH 4♥ 4D 2♥", 2808, HandRankName::TwoPair, HandRankClass::QueensAndFours)]
    #[case("QS QH 3♥ 3D AC", 2809, HandRankName::TwoPair, HandRankClass::QueensAndTreys)]
    #[case("QS QH 3♥ 3D 2♥", 2819, HandRankName::TwoPair, HandRankClass::QueensAndTreys)]
    #[case("QS QH 2♥ 2D AC", 2820, HandRankName::TwoPair, HandRankClass::QueensAndDeuces)]
    #[case("QS QH 2♥ 2D 3♥", 2830, HandRankName::TwoPair, HandRankClass::QueensAndDeuces)]
    #[case("JS JH T♥ TD AC", 2831, HandRankName::TwoPair, HandRankClass::JacksAndTens)]
    #[case("JS JH T♥ TD 2♥", 2841, HandRankName::TwoPair, HandRankClass::JacksAndTens)]
    #[case("JS JH 9♥ 9D AC", 2842, HandRankName::TwoPair, HandRankClass::JacksAndNines)]
    #[case("JS JH 9♥ 9D 2♥", 2852, HandRankName::TwoPair, HandRankClass::JacksAndNines)]
    #[case("JS JH 8♥ 8D AC", 2853, HandRankName::TwoPair, HandRankClass::JacksAndEights)]
    #[case("JS JH 8♥ 8D 2♥", 2863, HandRankName::TwoPair, HandRankClass::JacksAndEights)]
    #[case("JS JH 7♥ 7D AC", 2864, HandRankName::TwoPair, HandRankClass::JacksAndSevens)]
    #[case("JS JH 7♥ 7D 2♥", 2874, HandRankName::TwoPair, HandRankClass::JacksAndSevens)]
    #[case("JS JH 6♥ 6D AC", 2875, HandRankName::TwoPair, HandRankClass::JacksAndSixes)]
    #[case("JS JH 6♥ 6D 2♥", 2885, HandRankName::TwoPair, HandRankClass::JacksAndSixes)]
    #[case("JS JH 5♥ 5D AC", 2886, HandRankName::TwoPair, HandRankClass::JacksAndFives)]
    #[case("JS JH 5♥ 5D 2♥", 2896, HandRankName::TwoPair, HandRankClass::JacksAndFives)]
    #[case("JS JH 4♥ 4D AC", 2897, HandRankName::TwoPair, HandRankClass::JacksAndFours)]
    #[case("JS JH 4♥ 4D 2♥", 2907, HandRankName::TwoPair, HandRankClass::JacksAndFours)]
    #[case("JS JH 3♥ 3D AC", 2908, HandRankName::TwoPair, HandRankClass::JacksAndTreys)]
    #[case("JS JH 3♥ 3D 2♥", 2918, HandRankName::TwoPair, HandRankClass::JacksAndTreys)]
    #[case("JS JH 2♥ 2D AC", 2919, HandRankName::TwoPair, HandRankClass::JacksAndDeuces)]
    #[case("JS JH 2♥ 2D 3♥", 2929, HandRankName::TwoPair, HandRankClass::JacksAndDeuces)]
    #[case("TS TH 9♥ 9D AC", 2930, HandRankName::TwoPair, HandRankClass::TensAndNines)]
    #[case("TS TH 9♥ 9D 2♥", 2940, HandRankName::TwoPair, HandRankClass::TensAndNines)]
    #[case("TS TH 8♥ 8D AC", 2941, HandRankName::TwoPair, HandRankClass::TensAndEights)]
    #[case("TS TH 8♥ 8D 2♥", 2951, HandRankName::TwoPair, HandRankClass::TensAndEights)]
    #[case("TS TH 7♥ 7D AC", 2952, HandRankName::TwoPair, HandRankClass::TensAndSevens)]
    #[case("TS TH 7♥ 7D 2♥", 2962, HandRankName::TwoPair, HandRankClass::TensAndSevens)]
    #[case("TS TH 6♥ 6D AC", 2963, HandRankName::TwoPair, HandRankClass::TensAndSixes)]
    #[case("TS TH 6♥ 6D 2♥", 2973, HandRankName::TwoPair, HandRankClass::TensAndSixes)]
    #[case("TS TH 5♥ 5D AC", 2974, HandRankName::TwoPair, HandRankClass::TensAndFives)]
    #[case("TS TH 5♥ 5D 2♥", 2984, HandRankName::TwoPair, HandRankClass::TensAndFives)]
    #[case("TS TH 4♥ 4D AC", 2985, HandRankName::TwoPair, HandRankClass::TensAndFours)]
    #[case("TS TH 4♥ 4D 2♥", 2995, HandRankName::TwoPair, HandRankClass::TensAndFours)]
    #[case("TS TH 3♥ 3D AC", 2996, HandRankName::TwoPair, HandRankClass::TensAndTreys)]
    #[case("TS TH 3♥ 3D 2♥", 3006, HandRankName::TwoPair, HandRankClass::TensAndTreys)]
    #[case("TS TH 2♥ 2D AC", 3007, HandRankName::TwoPair, HandRankClass::TensAndDeuces)]
    #[case("TS TH 2♥ 2D 3♥", 3017, HandRankName::TwoPair, HandRankClass::TensAndDeuces)]
    #[case("9S 9H 8♥ 8D AC", 3018, HandRankName::TwoPair, HandRankClass::NinesAndEights)]
    #[case("9S 9H 8♥ 8D 2♥", 3028, HandRankName::TwoPair, HandRankClass::NinesAndEights)]
    #[case("9S 9H 7♥ 7D AC", 3029, HandRankName::TwoPair, HandRankClass::NinesAndSevens)]
    #[case("9S 9H 7♥ 7D 2♥", 3039, HandRankName::TwoPair, HandRankClass::NinesAndSevens)]
    #[case("9S 9H 6♥ 6D AC", 3040, HandRankName::TwoPair, HandRankClass::NinesAndSixes)]
    #[case("9S 9H 6♥ 6D 2♥", 3050, HandRankName::TwoPair, HandRankClass::NinesAndSixes)]
    #[case("9S 9H 5♥ 5D AC", 3051, HandRankName::TwoPair, HandRankClass::NinesAndFives)]
    #[case("9S 9H 5♥ 5D 2♥", 3061, HandRankName::TwoPair, HandRankClass::NinesAndFives)]
    #[case("9S 9H 4♥ 4D AC", 3062, HandRankName::TwoPair, HandRankClass::NinesAndFours)]
    #[case("9S 9H 4♥ 4D 2♥", 3072, HandRankName::TwoPair, HandRankClass::NinesAndFours)]
    #[case("9S 9H 3♥ 3D AC", 3073, HandRankName::TwoPair, HandRankClass::NinesAndTreys)]
    #[case("9S 9H 3♥ 3D 2♥", 3083, HandRankName::TwoPair, HandRankClass::NinesAndTreys)]
    #[case("9S 9H 2♥ 2D AC", 3084, HandRankName::TwoPair, HandRankClass::NinesAndDeuces)]
    #[case("9S 9H 2♥ 2D 3♥", 3094, HandRankName::TwoPair, HandRankClass::NinesAndDeuces)]
    #[case("8S 8H 7♥ 7D AC", 3095, HandRankName::TwoPair, HandRankClass::EightsAndSevens)]
    #[case("8S 8H 7♥ 7D 2♥", 3105, HandRankName::TwoPair, HandRankClass::EightsAndSevens)]
    #[case("8S 8H 6♥ 6D AC", 3106, HandRankName::TwoPair, HandRankClass::EightsAndSixes)]
    #[case("8S 8H 6♥ 6D 2♥", 3116, HandRankName::TwoPair, HandRankClass::EightsAndSixes)]
    #[case("8S 8H 5♥ 5D AC", 3117, HandRankName::TwoPair, HandRankClass::EightsAndFives)]
    #[case("8S 8H 5♥ 5D 2♥", 3127, HandRankName::TwoPair, HandRankClass::EightsAndFives)]
    #[case("8S 8H 4♥ 4D AC", 3128, HandRankName::TwoPair, HandRankClass::EightsAndFours)]
    #[case("8S 8H 4♥ 4D 2♥", 3138, HandRankName::TwoPair, HandRankClass::EightsAndFours)]
    #[case("8S 8H 3♥ 3D AC", 3139, HandRankName::TwoPair, HandRankClass::EightsAndTreys)]
    #[case("8S 8H 3♥ 3D 2♥", 3149, HandRankName::TwoPair, HandRankClass::EightsAndTreys)]
    #[case("8S 8H 2♥ 2D AC", 3150, HandRankName::TwoPair, HandRankClass::EightsAndDeuces)]
    #[case("8S 8H 2♥ 2D 3♥", 3160, HandRankName::TwoPair, HandRankClass::EightsAndDeuces)]
    #[case("7♥ 7D 6S 6C A♥", 3161, HandRankName::TwoPair, HandRankClass::SevensAndSixes)]
    #[case("7♥ 7D 6S 6♥ 2D", 3171, HandRankName::TwoPair, HandRankClass::SevensAndSixes)]
    #[case("7♥ 7D 5S 5C A♥", 3172, HandRankName::TwoPair, HandRankClass::SevensAndFives)]
    #[case("7♥ 7D 5S 5♥ 2D", 3182, HandRankName::TwoPair, HandRankClass::SevensAndFives)]
    #[case("7♥ 7D 4S 4C A♥", 3183, HandRankName::TwoPair, HandRankClass::SevensAndFours)]
    #[case("7♥ 7D 4S 4♥ 2D", 3193, HandRankName::TwoPair, HandRankClass::SevensAndFours)]
    #[case("7♥ 7D 3S 3C A♥", 3194, HandRankName::TwoPair, HandRankClass::SevensAndTreys)]
    #[case("7♥ 7D 3S 3♥ 2D", 3204, HandRankName::TwoPair, HandRankClass::SevensAndTreys)]
    #[case("7♥ 7D 2S 2C A♥", 3205, HandRankName::TwoPair, HandRankClass::SevensAndDeuces)]
    #[case("7♥ 7D 2S 2♥ 3D", 3215, HandRankName::TwoPair, HandRankClass::SevensAndDeuces)]
    #[case("6♥ 6D 5S 5C A♥", 3216, HandRankName::TwoPair, HandRankClass::SixesAndFives)]
    #[case("6♥ 6D 5S 5♥ 2D", 3226, HandRankName::TwoPair, HandRankClass::SixesAndFives)]
    #[case("6♥ 6D 4S 4C A♥", 3227, HandRankName::TwoPair, HandRankClass::SixesAndFours)]
    #[case("6♥ 6D 4S 4♥ 2D", 3237, HandRankName::TwoPair, HandRankClass::SixesAndFours)]
    #[case("6♥ 6D 3S 3C A♥", 3238, HandRankName::TwoPair, HandRankClass::SixesAndTreys)]
    #[case("6♥ 6D 3S 3♥ 2D", 3248, HandRankName::TwoPair, HandRankClass::SixesAndTreys)]
    #[case("6♥ 6D 2S 2C A♥", 3249, HandRankName::TwoPair, HandRankClass::SixesAndDeuces)]
    #[case("6♥ 6D 2S 2♥ 3D", 3259, HandRankName::TwoPair, HandRankClass::SixesAndDeuces)]
    #[case("5S 5C 4S 4D A♥", 3260, HandRankName::TwoPair, HandRankClass::FivesAndFours)]
    #[case("5S 5♥ 4S 4C 2D", 3270, HandRankName::TwoPair, HandRankClass::FivesAndFours)]
    #[case("5S 5C 3S 3D A♥", 3271, HandRankName::TwoPair, HandRankClass::FivesAndTreys)]
    #[case("5S 5♥ 3S 3C 2D", 3281, HandRankName::TwoPair, HandRankClass::FivesAndTreys)]
    #[case("5S 5C 2S 2D A♥", 3282, HandRankName::TwoPair, HandRankClass::FivesAndDeuces)]
    #[case("5S 5♥ 2S 2C 3D", 3292, HandRankName::TwoPair, HandRankClass::FivesAndDeuces)]
    #[case("4♥ 4D 3S 3C A♥", 3293, HandRankName::TwoPair, HandRankClass::FoursAndTreys)]
    #[case("4♥ 4D 3S 3♥ 2D", 3303, HandRankName::TwoPair, HandRankClass::FoursAndTreys)]
    #[case("4♥ 4D 2S 2C A♥", 3304, HandRankName::TwoPair, HandRankClass::FoursAndDeuces)]
    #[case("4♥ 4D 2S 2♥ 3D", 3314, HandRankName::TwoPair, HandRankClass::FoursAndDeuces)]
    #[case("3♥ 3D 2S 2C A♥", 3315, HandRankName::TwoPair, HandRankClass::TreysAndDeuces)]
    #[case("3♥ 3D 2S 2♥ 4D", 3325, HandRankName::TwoPair, HandRankClass::TreysAndDeuces)]
    #[case("A♥ AD KS Q♥ JD", 3326, HandRankName::Pair, HandRankClass::PairOfAces)]
    #[case("A♥ AD 4S 3♥ 2D", 3545, HandRankName::Pair, HandRankClass::PairOfAces)]
    #[case("K♥ KD AS Q♥ JD", 3546, HandRankName::Pair, HandRankClass::PairOfKings)]
    #[case("K♥ KD 4S 3♥ 2D", 3765, HandRankName::Pair, HandRankClass::PairOfKings)]
    #[case("Q♥ QD AS K♥ JD", 3766, HandRankName::Pair, HandRankClass::PairOfQueens)]
    #[case("Q♥ QD 4S 3♥ 2D", 3985, HandRankName::Pair, HandRankClass::PairOfQueens)]
    #[case("J♥ JD AS K♥ QD", 3986, HandRankName::Pair, HandRankClass::PairOfJacks)]
    #[case("J♥ JD 4S 3♥ 2D", 4205, HandRankName::Pair, HandRankClass::PairOfJacks)]
    #[case("T♥ TD AS K♥ QD", 4206, HandRankName::Pair, HandRankClass::PairOfTens)]
    #[case("T♥ TD 4S 3♥ 2D", 4425, HandRankName::Pair, HandRankClass::PairOfTens)]
    #[case("9♥ 9D AS K♥ QD", 4426, HandRankName::Pair, HandRankClass::PairOfNines)]
    #[case("9♥ 9D 4S 3♥ 2D", 4645, HandRankName::Pair, HandRankClass::PairOfNines)]
    #[case("8♥ 8D AS K♥ QD", 4646, HandRankName::Pair, HandRankClass::PairOfEights)]
    #[case("8♥ 8D 4S 3♥ 2D", 4865, HandRankName::Pair, HandRankClass::PairOfEights)]
    #[case("7♥ 7D AS K♥ QD", 4866, HandRankName::Pair, HandRankClass::PairOfSevens)]
    #[case("7♥ 7D 4S 3♥ 2D", 5085, HandRankName::Pair, HandRankClass::PairOfSevens)]
    #[case("6♥ 6D AS K♥ QD", 5086, HandRankName::Pair, HandRankClass::PairOfSixes)]
    #[case("6♥ 6D 4S 3♥ 2D", 5305, HandRankName::Pair, HandRankClass::PairOfSixes)]
    #[case("5♥ 5D AS K♥ QD", 5306, HandRankName::Pair, HandRankClass::PairOfFives)]
    #[case("5♥ 5D 4S 3♥ 2D", 5525, HandRankName::Pair, HandRankClass::PairOfFives)]
    #[case("4♥ 4D AS K♥ QD", 5526, HandRankName::Pair, HandRankClass::PairOfFours)]
    #[case("4♥ 4D 5S 3♥ 2D", 5745, HandRankName::Pair, HandRankClass::PairOfFours)]
    #[case("3♥ 3D AS K♥ QD", 5746, HandRankName::Pair, HandRankClass::PairOfTreys)]
    #[case("3♥ 3D 5S 4♥ 2D", 5965, HandRankName::Pair, HandRankClass::PairOfTreys)]
    #[case("2♥ 2D AS K♥ QD", 5966, HandRankName::Pair, HandRankClass::PairOfDeuces)]
    #[case("2♥ 2D 5S 4♥ 3D", 6185, HandRankName::Pair, HandRankClass::PairOfDeuces)]
    #[case("AD KD Q♥ JD 9D", 6186, HandRankName::HighCard, HandRankClass::AceHigh)]
    #[case("AD 6D 4♥ 3D 2D", 6678, HandRankName::HighCard, HandRankClass::AceHigh)]
    #[case("KD Q♥ JD TD 8C", 6679, HandRankName::HighCard, HandRankClass::KingHigh)]
    #[case("KD 5D 4♥ 3D 2D", 7007, HandRankName::HighCard, HandRankClass::KingHigh)]
    #[case("Q♥ JD TD 9C 7D", 7008, HandRankName::HighCard, HandRankClass::QueenHigh)]
    #[case("QD 5D 4♥ 3D 2D", 7216, HandRankName::HighCard, HandRankClass::QueenHigh)]
    #[case("JD TD 9C 8D 6C", 7217, HandRankName::HighCard, HandRankClass::JackHigh)]
    #[case("JD 5D 4♥ 3D 2D", 7341, HandRankName::HighCard, HandRankClass::JackHigh)]
    #[case("TD 9C 8D 7C 5S", 7342, HandRankName::HighCard, HandRankClass::TenHigh)]
    #[case("TD 5D 4♥ 3D 2D", 7410, HandRankName::HighCard, HandRankClass::TenHigh)]
    #[case("9C 8D 7C 6S 4D", 7411, HandRankName::HighCard, HandRankClass::NineHigh)]
    #[case("9D 5D 4♥ 3D 2D", 7444, HandRankName::HighCard, HandRankClass::NineHigh)]
    #[case("8D 7C 6S 5D 3H", 7445, HandRankName::HighCard, HandRankClass::EightHigh)]
    #[case("8D 5D 4♥ 3D 2D", 7458, HandRankName::HighCard, HandRankClass::EightHigh)]
    #[case("7C 6S 5D 4H 2C", 7459, HandRankName::HighCard, HandRankClass::SevenHigh)]
    #[case("7D 5D 4♥ 3D 2D", 7462, HandRankName::HighCard, HandRankClass::SevenHigh)]
    #[case("A♠ A♠ Q♠ J♠ T♠", 0, HandRankName::Invalid, HandRankClass::Invalid)]
    fn determine_class(
        #[case] index: &'static str,
        #[case] hand_rank_value: HandRankValue,
        #[case] hand_rank_name: HandRankName,
        #[case] hand_rank_class: HandRankClass,
    ) {
        let hand = Five::try_from(index).unwrap();

        let hand_rank = hand.hand_rank();

        assert_eq!(hand_rank_value, hand_rank.value);
        assert_eq!(hand_rank_name, hand_rank.name);
        assert_eq!(hand_rank_class, hand_rank.class);
        assert_eq!(HandRank::determine_class(&hand_rank_value), hand_rank_class);
    }

    /// This test verifies that an invalid hand passed in to be evaluated returns an invalid hand
    /// with a `HandRankValue` of zero. **NOTE:** not all invalid hand combinations have been
    /// verified.
    ///
    /// ```
    /// let invalid_5deuces_hand = CactusKevHand::from_index("2S 2H 2D 2C 2D").unwrap();
    /// assert!(invalid_5deuces_hand.is_invalid());
    /// ```
    #[test]
    fn panic_isolation() {
        let invalid_hand = five_from_index("2S 2H 2D 2C 2D").unwrap();

        let invalid_result = HandRank::from(crate::evaluate::five_cards(invalid_hand));

        assert_eq!(invalid_result, HandRank::default());
        assert!(invalid_result.is_invalid());

        // Restore the hook so other panics aren't suppressed.
        // panic::set_hook(hook);
    }
}
