/// A bitboard is represented as a 64 bit unsigned integer, 1 bit
/// per square for the standard 8x8 chess board. A 1 bit set indicates
/// the presence (or lack thereof) of some property for that square.
/// Properties include a particular piece being present, that square being
/// a potential target, etc... These bitboard are implmented in 
/// Little Endian Rank-File (LERF) order, meaning towards higher
/// valued bits we traverse first across a rank (the numbers), then up files (the letters).
/// 
/// To read more on bitboard representations, you can visit: 
/// <https://www.chessprogramming.org/Square_Mapping_Considerations>

use std::ops::{
    BitAnd, BitAndAssign, BitOr, 
    BitOrAssign, BitXor, BitXorAssign, 
    Shl, ShlAssign, Shr, ShrAssign, Not,

};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard(pub u64);

/// Constants used to initialize bitboards
pub const PAWN_START: Bitboard = Bitboard(0x00_ff_00_00_00_00_ff_00);
pub const KNIGHT_START: Bitboard = Bitboard(0x42_00_00_00_00_00_00_42);
pub const BISHOP_START: Bitboard = Bitboard(0x24_00_00_00_00_00_00_24);
pub const ROOK_START: Bitboard = Bitboard(0x81_00_00_00_00_00_00_81);
pub const QUEEN_START: Bitboard = Bitboard(0x08_00_00_00_00_00_00_08);
pub const KING_START: Bitboard = Bitboard(0x10_00_00_00_00_00_00_10);
pub const WHITE_START: Bitboard = Bitboard(0x00_00_00_00_00_00_ff_ff);
pub const BLACK_START: Bitboard = Bitboard(0xff_ff_00_00_00_00_00_00);
pub const EMPTY_START: Bitboard = Bitboard(0x00_00_ff_ff_ff_ff_00_00);
pub const OCCUPIED_START: Bitboard = Bitboard(0xff_ff_00_00_00_00_ff_ff);

/// 1s everywhere except for the A file
pub const NOT_A_FILE: Bitboard = Bitboard(0xfe_fe_fe_fe_fe_fe_fe_fe);
/// 1s everywhere except for the H file
pub const NOT_H_FILE: Bitboard = Bitboard(0x7f_7f_7f_7f_7f_7f_7f_7f);

/// Rank masks
pub const RANK4: Bitboard = Bitboard(0x00_00_00_00_ff_00_00_00);
pub const RANK5: Bitboard = Bitboard(0x00_00_00_ff_00_00_00_00);

/// PAWN_ATTACKS\[Color]\[Square] is a bitboard representing
/// where a pawn of that color on that square can attack
pub const PAWN_ATTACKS: [[Bitboard; 64]; 2] = [[Bitboard(512), Bitboard(1280), Bitboard(2560), Bitboard(5120), Bitboard(10240), Bitboard(20480), Bitboard(40960), Bitboard(16384), Bitboard(131072), Bitboard(327680), Bitboard(655360), Bitboard(1310720), Bitboard(2621440), Bitboard(5242880), Bitboard(10485760), Bitboard(4194304), Bitboard(33554432), Bitboard(83886080), Bitboard(167772160), Bitboard(335544320), Bitboard(671088640), Bitboard(1342177280), Bitboard(2684354560), Bitboard(1073741824), Bitboard(8589934592), Bitboard(21474836480), Bitboard(42949672960), Bitboard(85899345920), Bitboard(171798691840), Bitboard(343597383680), Bitboard(687194767360), Bitboard(274877906944), Bitboard(2199023255552), Bitboard(5497558138880), Bitboard(10995116277760), Bitboard(21990232555520), Bitboard(43980465111040), Bitboard(87960930222080), Bitboard(175921860444160), Bitboard(70368744177664), Bitboard(562949953421312), Bitboard(1407374883553280), Bitboard(2814749767106560), Bitboard(5629499534213120), Bitboard(11258999068426240), Bitboard(22517998136852480), Bitboard(45035996273704960), Bitboard(18014398509481984), Bitboard(144115188075855872), Bitboard(360287970189639680), Bitboard(720575940379279360), Bitboard(1441151880758558720), Bitboard(2882303761517117440), Bitboard(5764607523034234880), Bitboard(11529215046068469760), Bitboard(4611686018427387904), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)], [Bitboard(512), Bitboard(1024), Bitboard(2048), Bitboard(4096), Bitboard(8192), Bitboard(16384), Bitboard(32768), Bitboard(0), Bitboard(131072), Bitboard(262145), Bitboard(524290), Bitboard(1048580), Bitboard(2097160), Bitboard(4194320), Bitboard(8388640), Bitboard(64), Bitboard(33554432), Bitboard(67109120), Bitboard(134218240), Bitboard(268436480), Bitboard(536872960), Bitboard(1073745920), Bitboard(2147491840), Bitboard(16384), Bitboard(8589934592), Bitboard(17179934720), Bitboard(34359869440), Bitboard(68719738880), Bitboard(137439477760), Bitboard(274878955520), Bitboard(549757911040), Bitboard(4194304), Bitboard(2199023255552), Bitboard(4398063288320), Bitboard(8796126576640), Bitboard(17592253153280), Bitboard(35184506306560), Bitboard(70369012613120), Bitboard(140738025226240), Bitboard(1073741824), Bitboard(562949953421312), Bitboard(1125904201809920), Bitboard(2251808403619840), Bitboard(4503616807239680), Bitboard(9007233614479360), Bitboard(18014467228958720), Bitboard(36028934457917440), Bitboard(274877906944), Bitboard(144115188075855872), Bitboard(288231475663339520), Bitboard(576462951326679040), Bitboard(1152925902653358080), Bitboard(2305851805306716160), Bitboard(4611703610613432320), Bitboard(9223407221226864640), Bitboard(70368744177664), Bitboard(0), Bitboard(281474976710656), Bitboard(562949953421312), Bitboard(1125899906842624), Bitboard(2251799813685248), Bitboard(4503599627370496), Bitboard(9007199254740992), Bitboard(18014398509481984)]];

/// KING_ATTACKS\[Square] is a bitboard representing where a king on
/// that square can attack
pub const KING_ATTACKS: [Bitboard; 64] = [Bitboard(770), Bitboard(1797), Bitboard(3594), Bitboard(7188), Bitboard(14376), Bitboard(28752), Bitboard(57504), Bitboard(49216), Bitboard(197123), Bitboard(460039), Bitboard(920078), Bitboard(1840156), Bitboard(3680312), Bitboard(7360624), Bitboard(14721248), Bitboard(12599488), Bitboard(50463488), Bitboard(117769984), Bitboard(235539968), Bitboard(471079936), Bitboard(942159872), Bitboard(1884319744), Bitboard(3768639488), Bitboard(3225468928), Bitboard(12918652928), Bitboard(30149115904), Bitboard(60298231808), Bitboard(120596463616), Bitboard(241192927232), Bitboard(482385854464), Bitboard(964771708928), Bitboard(825720045568), Bitboard(3307175149568), Bitboard(7718173671424), Bitboard(15436347342848), Bitboard(30872694685696), Bitboard(61745389371392), Bitboard(123490778742784), Bitboard(246981557485568), Bitboard(211384331665408), Bitboard(846636838289408), Bitboard(1975852459884544), Bitboard(3951704919769088), Bitboard(7903409839538176), Bitboard(15806819679076352), Bitboard(31613639358152704), Bitboard(63227278716305408), Bitboard(54114388906344448), Bitboard(216739030602088448), Bitboard(505818229730443264), Bitboard(1011636459460886528), Bitboard(2023272918921773056), Bitboard(4046545837843546112), Bitboard(8093091675687092224), Bitboard(16186183351374184448), Bitboard(13853283560024178688), Bitboard(144959613005987840), Bitboard(362258295026614272), Bitboard(724516590053228544), Bitboard(1449033180106457088), Bitboard(2898066360212914176), Bitboard(5796132720425828352), Bitboard(11592265440851656704), Bitboard(4665729213955833856)];

/// KNIGHT_ATTACKS\[Square] is a bitboard representing where a king on
/// that square can attack
pub const KNIGHT_ATTACKS: [Bitboard; 64] = [Bitboard(132096), Bitboard(329728), Bitboard(659712), Bitboard(1319424), Bitboard(2638848), Bitboard(5277696), Bitboard(10489856), Bitboard(4202496), Bitboard(33816580), Bitboard(84410376), Bitboard(168886289), Bitboard(337772578), Bitboard(675545156), Bitboard(1351090312), Bitboard(2685403152), Bitboard(1075839008), Bitboard(8657044482), Bitboard(21609056261), Bitboard(43234889994), Bitboard(86469779988), Bitboard(172939559976), Bitboard(345879119952), Bitboard(687463207072), Bitboard(275414786112), Bitboard(2216203387392), Bitboard(5531918402816), Bitboard(11068131838464), Bitboard(22136263676928), Bitboard(44272527353856), Bitboard(88545054707712), Bitboard(175990581010432), Bitboard(70506185244672), Bitboard(567348067172352), Bitboard(1416171111120896), Bitboard(2833441750646784), Bitboard(5666883501293568), Bitboard(11333767002587136), Bitboard(22667534005174272), Bitboard(45053588738670592), Bitboard(18049583422636032), Bitboard(145241105196122112), Bitboard(362539804446949376), Bitboard(725361088165576704), Bitboard(1450722176331153408), Bitboard(2901444352662306816), Bitboard(5802888705324613632), Bitboard(11533718717099671552), Bitboard(4620693356194824192), Bitboard(288234782788157440), Bitboard(576469569871282176), Bitboard(1224997833292120064), Bitboard(2449995666584240128), Bitboard(4899991333168480256), Bitboard(9799982666336960512), Bitboard(1152939783987658752), Bitboard(2305878468463689728), Bitboard(1128098930098176), Bitboard(2257297371824128), Bitboard(4796069720358912), Bitboard(9592139440717824), Bitboard(19184278881435648), Bitboard(38368557762871296), Bitboard(4679521487814656), Bitboard(9077567998918656)];
/// RAY_ATTACKS/[dir]/[square] is a bitboard representing an attack
/// ray in that direction. Directions go from low to high indices in the
/// following order: north, northeast, east, southeast, south, southwest,
/// west, northwest.
pub const RAY_ATTACKS: [[Bitboard; 64]; 8] = [[Bitboard(72340172838076672), Bitboard(144680345676153344), Bitboard(289360691352306688), Bitboard(578721382704613376), Bitboard(1157442765409226752), Bitboard(2314885530818453504), Bitboard(4629771061636907008), Bitboard(9259542123273814016), Bitboard(72340172838076416), Bitboard(144680345676152832), Bitboard(289360691352305664), Bitboard(578721382704611328), Bitboard(1157442765409222656), Bitboard(2314885530818445312), Bitboard(4629771061636890624), Bitboard(9259542123273781248), Bitboard(72340172838010880), Bitboard(144680345676021760), Bitboard(289360691352043520), Bitboard(578721382704087040), Bitboard(1157442765408174080), Bitboard(2314885530816348160), Bitboard(4629771061632696320), Bitboard(9259542123265392640), Bitboard(72340172821233664), Bitboard(144680345642467328), Bitboard(289360691284934656), Bitboard(578721382569869312), Bitboard(1157442765139738624), Bitboard(2314885530279477248), Bitboard(4629771060558954496), Bitboard(9259542121117908992), Bitboard(72340168526266368), Bitboard(144680337052532736), Bitboard(289360674105065472), Bitboard(578721348210130944), Bitboard(1157442696420261888), Bitboard(2314885392840523776), Bitboard(4629770785681047552), Bitboard(9259541571362095104), Bitboard(72339069014638592), Bitboard(144678138029277184), Bitboard(289356276058554368), Bitboard(578712552117108736), Bitboard(1157425104234217472), Bitboard(2314850208468434944), Bitboard(4629700416936869888), Bitboard(9259400833873739776), Bitboard(72057594037927936), Bitboard(144115188075855872), Bitboard(288230376151711744), Bitboard(576460752303423488), Bitboard(1152921504606846976), Bitboard(2305843009213693952), Bitboard(4611686018427387904), Bitboard(9223372036854775808), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)], [Bitboard(9241421688590303744), Bitboard(36099303471055872), Bitboard(141012904183808), Bitboard(550831656960), Bitboard(2151686144), Bitboard(8404992), Bitboard(32768), Bitboard(0), Bitboard(4620710844295151616), Bitboard(9241421688590303232), Bitboard(36099303471054848), Bitboard(141012904181760), Bitboard(550831652864), Bitboard(2151677952), Bitboard(8388608), Bitboard(0), Bitboard(2310355422147510272), Bitboard(4620710844295020544), Bitboard(9241421688590041088), Bitboard(36099303470530560), Bitboard(141012903133184), Bitboard(550829555712), Bitboard(2147483648), Bitboard(0), Bitboard(1155177711056977920), Bitboard(2310355422113955840), Bitboard(4620710844227911680), Bitboard(9241421688455823360), Bitboard(36099303202095104), Bitboard(141012366262272), Bitboard(549755813888), Bitboard(0), Bitboard(577588851233521664), Bitboard(1155177702467043328), Bitboard(2310355404934086656), Bitboard(4620710809868173312), Bitboard(9241421619736346624), Bitboard(36099165763141632), Bitboard(140737488355328), Bitboard(0), Bitboard(288793326105133056), Bitboard(577586652210266112), Bitboard(1155173304420532224), Bitboard(2310346608841064448), Bitboard(4620693217682128896), Bitboard(9241386435364257792), Bitboard(36028797018963968), Bitboard(0), Bitboard(144115188075855872), Bitboard(288230376151711744), Bitboard(576460752303423488), Bitboard(1152921504606846976), Bitboard(2305843009213693952), Bitboard(4611686018427387904), Bitboard(9223372036854775808), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)], [Bitboard(254), Bitboard(252), Bitboard(248), Bitboard(240), Bitboard(224), Bitboard(192), Bitboard(128), Bitboard(0), Bitboard(65024), Bitboard(64512), Bitboard(63488), Bitboard(61440), Bitboard(57344), Bitboard(49152), Bitboard(32768), Bitboard(0), Bitboard(16646144), Bitboard(16515072), Bitboard(16252928), Bitboard(15728640), Bitboard(14680064), Bitboard(12582912), Bitboard(8388608), Bitboard(0), Bitboard(4261412864), Bitboard(4227858432), Bitboard(4160749568), Bitboard(4026531840), Bitboard(3758096384), Bitboard(3221225472), Bitboard(2147483648), Bitboard(0), Bitboard(1090921693184), Bitboard(1082331758592), Bitboard(1065151889408), Bitboard(1030792151040), Bitboard(962072674304), Bitboard(824633720832), Bitboard(549755813888), Bitboard(0), Bitboard(279275953455104), Bitboard(277076930199552), Bitboard(272678883688448), Bitboard(263882790666240), Bitboard(246290604621824), Bitboard(211106232532992), Bitboard(140737488355328), Bitboard(0), Bitboard(71494644084506624), Bitboard(70931694131085312), Bitboard(69805794224242688), Bitboard(67553994410557440), Bitboard(63050394783186944), Bitboard(54043195528445952), Bitboard(36028797018963968), Bitboard(0), Bitboard(18302628885633695744), Bitboard(18158513697557839872), Bitboard(17870283321406128128), Bitboard(17293822569102704640), Bitboard(16140901064495857664), Bitboard(13835058055282163712), Bitboard(9223372036854775808), Bitboard(0)], [Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(2), Bitboard(4), Bitboard(8), Bitboard(16), Bitboard(32), Bitboard(64), Bitboard(128), Bitboard(0), Bitboard(516), Bitboard(1032), Bitboard(2064), Bitboard(4128), Bitboard(8256), Bitboard(16512), Bitboard(32768), Bitboard(0), Bitboard(132104), Bitboard(264208), Bitboard(528416), Bitboard(1056832), Bitboard(2113664), Bitboard(4227072), Bitboard(8388608), Bitboard(0), Bitboard(33818640), Bitboard(67637280), Bitboard(135274560), Bitboard(270549120), Bitboard(541097984), Bitboard(1082130432), Bitboard(2147483648), Bitboard(0), Bitboard(8657571872), Bitboard(17315143744), Bitboard(34630287488), Bitboard(69260574720), Bitboard(138521083904), Bitboard(277025390592), Bitboard(549755813888), Bitboard(0), Bitboard(2216338399296), Bitboard(4432676798592), Bitboard(8865353596928), Bitboard(17730707128320), Bitboard(35461397479424), Bitboard(70918499991552), Bitboard(140737488355328), Bitboard(0), Bitboard(567382630219904), Bitboard(1134765260439552), Bitboard(2269530520813568), Bitboard(4539061024849920), Bitboard(9078117754732544), Bitboard(18155135997837312), Bitboard(36028797018963968), Bitboard(0)], [Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(1), Bitboard(2), Bitboard(4), Bitboard(8), Bitboard(16), Bitboard(32), Bitboard(64), Bitboard(128), Bitboard(257), Bitboard(514), Bitboard(1028), Bitboard(2056), Bitboard(4112), Bitboard(8224), Bitboard(16448), Bitboard(32896), Bitboard(65793), Bitboard(131586), Bitboard(263172), Bitboard(526344), Bitboard(1052688), Bitboard(2105376), Bitboard(4210752), Bitboard(8421504), Bitboard(16843009), Bitboard(33686018), Bitboard(67372036), Bitboard(134744072), Bitboard(269488144), Bitboard(538976288), Bitboard(1077952576), Bitboard(2155905152), Bitboard(4311810305), Bitboard(8623620610), Bitboard(17247241220), Bitboard(34494482440), Bitboard(68988964880), Bitboard(137977929760), Bitboard(275955859520), Bitboard(551911719040), Bitboard(1103823438081), Bitboard(2207646876162), Bitboard(4415293752324), Bitboard(8830587504648), Bitboard(17661175009296), Bitboard(35322350018592), Bitboard(70644700037184), Bitboard(141289400074368), Bitboard(282578800148737), Bitboard(565157600297474), Bitboard(1130315200594948), Bitboard(2260630401189896), Bitboard(4521260802379792), Bitboard(9042521604759584), Bitboard(18085043209519168), Bitboard(36170086419038336)], [Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(1), Bitboard(2), Bitboard(4), Bitboard(8), Bitboard(16), Bitboard(32), Bitboard(64), Bitboard(0), Bitboard(256), Bitboard(513), Bitboard(1026), Bitboard(2052), Bitboard(4104), Bitboard(8208), Bitboard(16416), Bitboard(0), Bitboard(65536), Bitboard(131328), Bitboard(262657), Bitboard(525314), Bitboard(1050628), Bitboard(2101256), Bitboard(4202512), Bitboard(0), Bitboard(16777216), Bitboard(33619968), Bitboard(67240192), Bitboard(134480385), Bitboard(268960770), Bitboard(537921540), Bitboard(1075843080), Bitboard(0), Bitboard(4294967296), Bitboard(8606711808), Bitboard(17213489152), Bitboard(34426978560), Bitboard(68853957121), Bitboard(137707914242), Bitboard(275415828484), Bitboard(0), Bitboard(1099511627776), Bitboard(2203318222848), Bitboard(4406653222912), Bitboard(8813306511360), Bitboard(17626613022976), Bitboard(35253226045953), Bitboard(70506452091906), Bitboard(0), Bitboard(281474976710656), Bitboard(564049465049088), Bitboard(1128103225065472), Bitboard(2256206466908160), Bitboard(4512412933881856), Bitboard(9024825867763968), Bitboard(18049651735527937)], [Bitboard(0), Bitboard(1), Bitboard(3), Bitboard(7), Bitboard(15), Bitboard(31), Bitboard(63), Bitboard(127), Bitboard(0), Bitboard(256), Bitboard(768), Bitboard(1792), Bitboard(3840), Bitboard(7936), Bitboard(16128), Bitboard(32512), Bitboard(0), Bitboard(65536), Bitboard(196608), Bitboard(458752), Bitboard(983040), Bitboard(2031616), Bitboard(4128768), Bitboard(8323072), Bitboard(0), Bitboard(16777216), Bitboard(50331648), Bitboard(117440512), Bitboard(251658240), Bitboard(520093696), Bitboard(1056964608), Bitboard(2130706432), Bitboard(0), Bitboard(4294967296), Bitboard(12884901888), Bitboard(30064771072), Bitboard(64424509440), Bitboard(133143986176), Bitboard(270582939648), Bitboard(545460846592), Bitboard(0), Bitboard(1099511627776), Bitboard(3298534883328), Bitboard(7696581394432), Bitboard(16492674416640), Bitboard(34084860461056), Bitboard(69269232549888), Bitboard(139637976727552), Bitboard(0), Bitboard(281474976710656), Bitboard(844424930131968), Bitboard(1970324836974592), Bitboard(4222124650659840), Bitboard(8725724278030336), Bitboard(17732923532771328), Bitboard(35747322042253312), Bitboard(0), Bitboard(72057594037927936), Bitboard(216172782113783808), Bitboard(504403158265495552), Bitboard(1080863910568919040), Bitboard(2233785415175766016), Bitboard(4539628424389459968), Bitboard(9151314442816847872)], [Bitboard(0), Bitboard(256), Bitboard(66048), Bitboard(16909312), Bitboard(4328785920), Bitboard(1108169199616), Bitboard(283691315109888), Bitboard(72624976668147712), Bitboard(0), Bitboard(65536), Bitboard(16908288), Bitboard(4328783872), Bitboard(1108169195520), Bitboard(283691315101696), Bitboard(72624976668131328), Bitboard(145249953336262656), Bitboard(0), Bitboard(16777216), Bitboard(4328521728), Bitboard(1108168671232), Bitboard(283691314053120), Bitboard(72624976666034176), Bitboard(145249953332068352), Bitboard(290499906664136704), Bitboard(0), Bitboard(4294967296), Bitboard(1108101562368), Bitboard(283691179835392), Bitboard(72624976397598720), Bitboard(145249952795197440), Bitboard(290499905590394880), Bitboard(580999811180789760), Bitboard(0), Bitboard(1099511627776), Bitboard(283673999966208), Bitboard(72624942037860352), Bitboard(145249884075720704), Bitboard(290499768151441408), Bitboard(580999536302882816), Bitboard(1161999072605765632), Bitboard(0), Bitboard(281474976710656), Bitboard(72620543991349248), Bitboard(145241087982698496), Bitboard(290482175965396992), Bitboard(580964351930793984), Bitboard(1161928703861587968), Bitboard(2323857407723175936), Bitboard(0), Bitboard(72057594037927936), Bitboard(144115188075855872), Bitboard(288230376151711744), Bitboard(576460752303423488), Bitboard(1152921504606846976), Bitboard(2305843009213693952), Bitboard(4611686018427387904), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0), Bitboard(0)]];

impl Bitboard {
    /// Shifts the bitboard `b` south one
    pub fn sout_one(b: Bitboard) -> Bitboard {
        b >> 8
    }

    /// Shifts the bitboard `b` north one
    pub fn nort_one(b: Bitboard) -> Bitboard {
        b << 8
    }

    /// Shifts the bitboard `b` east one
    pub fn east_one(b: Bitboard) -> Bitboard {
        (b << 1) & self::NOT_A_FILE
    }

    /// Shifts the bitboard `b` west one
    pub fn west_one(b: Bitboard) -> Bitboard {
        (b >> 1) & self::NOT_H_FILE
    }

    /// Shifts the bitboard `b` northeast one
    pub fn noea_one(b: Bitboard) -> Bitboard {
        (b << 9) & self::NOT_A_FILE
    }

    /// Shifts the bitboard `b` southeast one
    pub fn soea_one(b: Bitboard) -> Bitboard {
        (b >> 7) & self::NOT_A_FILE
    }

    /// Shifts the bitboard `b` southwest one
    pub fn sowe_one(b: Bitboard) -> Bitboard {
        (b >> 9) & self::NOT_H_FILE
    }

    /// Shifts the bitboard `b` northwest one
    pub fn nowe_one(b: Bitboard) -> Bitboard {
        (b << 7) & self::NOT_H_FILE
    }

    /// Rotates the bitboard `b` to the left by `s` bits
    pub fn rotate_left(Bitboard(b): Bitboard, s: u32) -> Bitboard {
        Bitboard(b.rotate_left(s))
    }

    /// Rotates the bitboard `b` to the right by `s` bits
    pub fn rotate_right(Bitboard(b): Bitboard, s: u32) -> Bitboard {
        Bitboard(b.rotate_right(s))
    }
    /// Returns the index of the least significant 1 bit, or None
    /// if there is no 1 bit
    pub fn bit_scan(&self) -> Option<u32> {
        let trailing_zeros = self.0.trailing_zeros();
        if trailing_zeros == 64 { None } else { Some(trailing_zeros) }
    }

    /// Returns the index of the most significant 1 bit, or None
    /// if there is no 1 bit
    pub fn bit_scan_reverse(&self) -> Option<u32> {
        let leading_zeros = self.0.leading_zeros();
        if leading_zeros == 64 { None } else { Some(leading_zeros) }
    }
}

impl ToString for Bitboard {
    fn to_string(&self) -> String {
        let mut res = String::from("");
        for i in (0..8).rev() {
            let rank = (self.0 >> (i * 8)) & 0xff;
            for j in 0..8 {
                let square = (rank >> j) & 1;
                if square == 1 {
                    res += "1 ";
                } else {
                    res += ". ";
                }
            }
            res += "\n";
        }
        return res;
    }
}

// Implementation of bitwise operations for bitboards
// Just use the operation on the inner u64
impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Shl<i32> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl ShlAssign<i32> for Bitboard {
    fn shl_assign(&mut self, rhs: i32) {
        *self = Self(self.0 << rhs)
    }
}

impl Shr<i32> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl ShrAssign<i32> for Bitboard {
    fn shr_assign(&mut self, rhs: i32) {
        *self = Self(self.0 >> rhs)
    }
}
