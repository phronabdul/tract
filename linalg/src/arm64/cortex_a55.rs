use crate::frame::mmm::CostModel;
        pub fn model() -> CostModel<'static> {
            CostModel {
                big_product_mkn_threshold: 263214080.0,
                big_product_kernel_choice: "arm64simd_mmm_f32_12x8_a55",
                kernels: &["arm64simd_mmm_f32_12x8_a53", "arm64simd_mmm_f32_12x8_a55", "arm64simd_mmm_f32_12x8_gen", "arm64simd_mmm_f32_16x4_a53", "arm64simd_mmm_f32_16x4_a55", "arm64simd_mmm_f32_16x4_gen", "arm64simd_mmm_f32_24x4_a53", "arm64simd_mmm_f32_24x4_a55", "arm64simd_mmm_f32_24x4_gen", "arm64simd_mmm_f32_8x8_a53", "arm64simd_mmm_f32_8x8_a55", "arm64simd_mmm_f32_8x8_gen", "generic_f32_4x4"],
                mrs: &[4, 8, 12, 16, 24],
                nrs: &[4, 8],
                feat_norm_mean: &[5.27886946965165, 6.250454700699139, 5.241114620514529, 16.770438790865423, 1.540625, 0.770625, 3.518125, 0.8775, 5.560625, 0.923125, 7.453125, 0.943125, 11.613125, 0.9575, 1.509375, 0.771875, 3.581875, 0.898125],
                feat_norm_stddev: &[0.9509890252368828, 0.6930410342704738, 1.0261938261805659, 1.600617293156687, 1.0981118382819681, 0.42043086158725473, 2.338198341538852, 0.3278623949159141, 3.494112850120205, 0.26639300736881577, 4.56457037785321, 0.2316036147710134, 7.043415558830455, 0.20172691937369452, 1.0925484471523377, 0.4196236222795381, 2.273113830052299, 0.30248385804039707],
                w1: &[-0.13682155311107635, -0.1783919334411621, 0.26539096236228943, 0.19552235305309296, -0.10618806630373001, 0.13501706719398499, 0.21776071190834045, 0.08390733599662781, -0.2215081751346588, 0.18829140067100525, -0.20535176992416382, -0.0463368222117424, 0.05815611779689789, -0.13855215907096863, -0.024539709091186523, -0.4855460524559021, -0.414151668548584, -0.7574286460876465, 0.8987273573875427, 0.5316352844238281, 0.8244147896766663, 0.8388808369636536, -0.02545193023979664, 0.04357631504535675, -0.007071307860314846, 0.18223997950553894, -0.04292978346347809, 0.004330582916736603, -0.013073648326098919, -0.04028080403804779, -0.09901119023561478, 0.062175191938877106, -0.006247916258871555, 0.009531030431389809, 0.09731218218803406, 0.004297865089029074, 0.6260067224502563, -0.10042139887809753, 0.807989239692688, 0.6866835951805115, -0.018399836495518684, -0.07194910198450089, -0.18889868259429932, -0.07729395478963852, -0.03907148540019989, 0.017019111663103104, 0.06159460172057152, -0.02395886555314064, 0.23730705678462982, -0.15546496212482452, -0.04492897167801857, -0.003982013091444969, -0.09160511195659637, 0.03185845538973808, -0.27577653527259827, 0.5699247121810913, -0.6027079820632935, -0.4136800467967987, -0.04364140331745148, -0.11226192861795425, 0.16899903118610382, -0.11524038016796112, 0.12308179587125778, 0.027925828471779823, -0.06269104778766632, 0.11644940823316574, -0.369202196598053, 0.3338239789009094, -0.06509242206811905, 0.2303273230791092, 0.018171854317188263, -0.08709719777107239, 0.18228614330291748, -0.071574367582798, -0.012407014146447182, -0.284942090511322, -0.1326635330915451, -0.08634418249130249, 0.11018547415733337, -0.09423547983169556, 0.13697068393230438, -0.03515861555933952, 0.014629656448960304, -0.21159854531288147, 0.15693463385105133, -0.021487032994627953, 0.032396819442510605, 0.028369005769491196, 0.08724819868803024, -0.13204769790172577, 0.4691336452960968, 0.1237262561917305, -0.06020978465676308, 0.24037614464759827, 0.05237792432308197, -0.10641840100288391, 0.1820996105670929, 0.6079273819923401, -0.4903985857963562, 0.40744978189468384, 0.43370547890663147, 0.5092437863349915, 0.0810965895652771, 0.4670366048812866, -0.11692337691783905, -0.013550599105656147, -0.364605575799942, 0.34470170736312866, -0.01755279302597046, 0.30621764063835144, 0.35784396529197693, 0.42736300826072693, 0.022546129301190376, 0.08497388660907745, 0.07601173967123032, 0.0696730837225914, -0.21918217837810516, 0.6236687898635864, -0.0793512761592865, -0.0668395534157753, 0.010559543035924435, 0.46621084213256836, -0.2632196843624115, -0.03991322219371796, 0.1392994076013565, 0.003188274335116148, -0.2655166983604431, -0.22143644094467163, -0.19157607853412628, -0.39395904541015625, -0.021266555413603783, 0.08848410844802856, 0.08152330666780472, 0.013220606371760368, -0.18424198031425476, 0.05234640836715698, 0.05919161066412926, -0.16255362331867218, -0.04549096152186394, 0.044437166303396225, 0.21704396605491638, -0.5149197578430176, -0.6047705411911011, -0.8048356175422668, -0.36901935935020447, -0.19035962224006653, 1.3252514600753784, 0.19824109971523285, 0.07630860805511475, -0.011165079660713673, 0.011559495702385902, 0.10554458945989609, -0.12820255756378174, 0.29352235794067383, -0.06662449240684509, -0.15792854130268097, 0.0345480814576149, -0.04881494492292404, -0.06912268698215485, -0.00013739474525209516, -0.1597173660993576, 0.34323570132255554, 0.34446775913238525, 0.3795395493507385, 0.017453864216804504, 0.1102253794670105, 0.04026523232460022, 0.1107630804181099, 0.10295291990041733, 0.5326219201087952, 0.1749747395515442, -0.2661803066730499, 0.11752097308635712, 0.08010037988424301, -0.5501991510391235, -0.059987347573041916, -0.04125819355249405, 0.16356444358825684, 0.020170046016573906, -0.08306766301393509, 0.17777052521705627, 0.4687126874923706, 0.7723219394683838, 0.7309747934341431, -0.019829215481877327, 0.10945341736078262, 0.06796073168516159, -0.12042505294084549, -0.26762208342552185, 0.10846878588199615, 0.013867417350411415, 0.01077105849981308, -0.10193657130002975, -0.1757654845714569, -0.245382159948349, 0.20442898571491241, 0.10115985572338104, 0.2514199912548065, -0.3793720304965973, -0.6926521062850952, -0.6686761975288391, -0.607191264629364, 0.16187654435634613, -0.0073340232484042645, -0.09948350489139557, -0.21431320905685425, -0.12334707379341125, -0.15290899574756622, -0.026063116267323494, 0.26553207635879517, 0.18921764194965363, -0.1665697544813156, -0.00264778733253479, 0.20274107158184052, 0.3660823404788971, -0.35731416940689087, 0.50246661901474, 0.2781502604484558, 0.1629776805639267, -0.03493829071521759, -0.16012291610240936, -0.08139592409133911, -0.1440155804157257, 0.32721832394599915, 0.1312151998281479, 0.17874418199062347, 0.06143738701939583, -0.05158458650112152, 0.4802771806716919, -0.6857288479804993, 0.08245638012886047, -0.09577414393424988, -0.12872998416423798, 0.16155612468719482, 0.24089869856834412, 0.44030725955963135, -0.30994167923927307, 0.12139064073562622, 0.029418930411338806, -0.051672156900167465, -0.10080718994140625, -0.007311842869967222, -0.15189751982688904, -0.1559375822544098, 0.2731820344924927, -0.03627878054976463, 0.10538394004106522, 0.15048423409461975, 0.12981411814689636, 0.0002639668236952275, 0.05666665732860565, 0.08173252642154694, -0.16131722927093506, -0.043261025100946426, -0.14845971763134003, -0.29335740208625793, 0.039398159831762314, -0.02791670151054859, 0.22897064685821533, -0.12178067117929459, -0.4062419831752777, 0.3934949040412903, -0.05093907564878464, -0.06126153841614723, -0.07318481802940369, -0.08793392032384872, -0.01818496361374855, -0.24753189086914062, -0.30580347776412964, 0.44876909255981445, 0.5379880666732788, 0.11587893962860107, 0.2174995243549347, -0.035063862800598145, -0.0010147193679586053, -0.12281838059425354, -0.21301835775375366, 0.3645245432853699, 0.39920729398727417, -0.45564430952072144, 0.03503882512450218, 0.6949061155319214, -0.5742982625961304, 0.38680514693260193, -0.018345845863223076, 0.04529440030455589, -0.04468340799212456, -0.020917288959026337, 0.2523670792579651, -0.4574699103832245, 0.17178472876548767, -0.12147565186023712, 0.043810319155454636, -0.17998050153255463, -0.09663069248199463, -0.03498067706823349, 0.06111514940857887, -0.11410824209451675, 0.18208050727844238, -0.09109053015708923, 0.08489643037319183, 0.15014725923538208, 0.18506401777267456, -0.060843177139759064, -0.11932594329118729, 0.11290943622589111, -0.23226700723171234, -0.2114422470331192, -0.36001038551330566, -0.29864072799682617, -0.05599717050790787, -0.21294310688972473, -0.1301364004611969, -0.4993196725845337, 0.097460076212883, 0.030209479853510857, 0.35134217143058777, -0.9156147837638855, 0.0173207875341177, -0.9142565131187439, 0.13512593507766724, -0.1926516443490982, -0.2812888026237488, 0.04805266484618187, 0.5790673494338989, -0.28300249576568604, -0.10372477024793625, 0.2964925169944763, 0.16425621509552002, -0.25588271021842957, 0.37744808197021484, -0.07827199995517731, -0.7785226702690125, -0.4873232841491699, -0.0240982286632061, -0.31732890009880066, -0.7271391749382019, -0.40648236870765686, -0.08706668019294739, -0.0876365602016449, -0.08107846975326538, 0.049622420221567154, 0.5049374103546143, -0.09109669923782349, -0.2958216369152069, 0.23400314152240753, 0.0727144181728363, -0.06163109838962555, -0.3235352635383606, -0.08323507010936737, 0.06926267594099045, 0.12505480647087097, 0.06806384027004242, -0.1783592253923416, -0.09036792814731598, 0.007250780239701271, 0.07478834688663483, 0.37752634286880493, 0.10522382706403732, -0.3126020133495331, -0.339804470539093, -0.2922729253768921, -0.04612985998392105, 0.06431944668292999, 0.08483731746673584, 0.12883307039737701, -0.015924949198961258, 0.10468991845846176, -0.3394957184791565, 0.23376204073429108, -0.22720825672149658, 0.005506275221705437, -0.22926953434944153, -0.10148110240697861, 0.06526672840118408, -0.2586720287799835, -0.32853958010673523, 0.3440588712692261, -0.11197478324174881, -0.24647162854671478, 0.32472386956214905, 0.18955329060554504, 0.22783295810222626, 0.27004650235176086, 0.06792190670967102, -0.25404539704322815, -0.0421239472925663, 0.19141103327274323, -0.1919824779033661, 0.024490466341376305, -0.45774775743484497, 0.15080632269382477, -0.21607035398483276, -0.15506379306316376, -0.4421549439430237, -0.3747740089893341, -0.40712970495224, -0.01002188865095377, -0.18514835834503174, -0.052659012377262115, -0.009491002187132835, -0.04560127854347229, 0.5816720724105835, -0.8684999942779541, -0.6074734330177307, -0.6023196578025818, 0.09026342630386353, -0.8521136045455933, -0.677777886390686, -0.7927519083023071, 0.05012498050928116, 0.006620208732783794, 0.09600439667701721, 0.006934305187314749, -0.41822823882102966, 0.5416979193687439, 1.3451576232910156, 0.6131516098976135, -0.1447380781173706, 0.09429032355546951, 0.06888633966445923, 0.09988542646169662, -0.09572823345661163, 0.09141702950000763, 0.05828794091939926, -0.20784544944763184, -0.14200495183467865, 0.014049896970391273, -0.081334687769413, 0.15918458998203278, 0.001768372836522758, 0.009856577031314373, 0.5256384611129761, 0.49961280822753906, 0.5969673991203308, 0.37020817399024963, -0.07463415712118149, -0.0038648881018161774, 0.014317997731268406, 0.07256675511598587, 0.27220791578292847, -0.14287996292114258, -0.18170645833015442, -0.021593274548649788, -0.15909305214881897, 0.3259168863296509, -0.11064229905605316, 0.12034989148378372, 0.36166661977767944, -0.21680544316768646, -0.14505243301391602, -0.24518895149230957, -0.054052721709012985, 0.11477477848529816, 0.10946492105722427, -0.004644579254090786, -0.11873581260442734, 0.00934956781566143, 0.026955196633934975, -0.0947655513882637, -0.0432097427546978, 0.2264525443315506, 0.4585563540458679, -0.2117093950510025, 0.06864829361438751, 0.01817937195301056, -0.09130346775054932, -0.031736359000205994, -0.6623827219009399, 0.07924489676952362, 0.30316102504730225, 0.06474705785512924, 0.12052184343338013, -0.06878554821014404, 0.048135798424482346, 0.14442582428455353, -0.1945008486509323, 0.16308918595314026, 0.13180820643901825, -0.3005691170692444, -0.08318639546632767, -0.0371159091591835, -0.036223117262125015, 0.27411049604415894, -0.008904200047254562, -0.21584218740463257, -0.22458405792713165, -0.2840893864631653, 0.9380438327789307, -0.026274412870407104, -0.03674294427037239, -0.039288733154535294, 0.20259428024291992, -0.2627299726009369, -0.03588804602622986, -0.09061996638774872, 0.0026293552946299314, -1.1599351167678833, -0.0888570249080658, 0.3020864427089691, 0.10419020056724548, -0.2301473766565323, -0.2372182309627533, 0.255910724401474, -0.9108321666717529, -0.17266617715358734, -0.21715109050273895, -0.4768790900707245, 0.02349638193845749, 0.06996935606002808, 0.2306048572063446, -0.2647320032119751, -0.5029106140136719, 0.18124276399612427, 0.05404527485370636, -0.556660532951355, -0.20282964408397675, 0.1787903904914856, -0.13809867203235626, 0.012665750458836555, -0.007909105159342289, -0.11666542291641235, 0.192016139626503, 0.20280246436595917, 0.04091315343976021, 0.21129484474658966, 0.06015581637620926, -0.1396055370569229, 0.11048803478479385, -0.22130873799324036, 0.10175041109323502, 0.15478093922138214, -0.06699641793966293, 0.16655825078487396, -0.5767931938171387, 0.23376262187957764, -0.06561370939016342, 0.08572515100240707, 0.22690269351005554, -0.10714394599199295, 0.2328615039587021, 0.06609856337308884, 0.15064586699008942, 0.1398843675851822, 9.159173350781202e-05, -0.006412057671695948, 0.1231503039598465, 0.2868848741054535, -0.37850138545036316, -0.4390513002872467, -0.10716433078050613, -0.16492293775081635, -0.17774488031864166, -0.006263014394789934, -0.15535981953144073, -0.15121980011463165, -0.022719506174325943, -0.3260766863822937, 0.1365034133195877, 0.7772430777549744, 0.8306354880332947, 0.8039601445198059, 0.16534824669361115, -0.03939266875386238, -0.15611104667186737, 0.21217003464698792, -0.022034769877791405, -0.025939559563994408, 0.1058378517627716, -0.08505864441394806, 0.08503950387239456, -0.0037705348804593086, -0.0026697057764977217, 0.3492349088191986, 0.15157155692577362, -0.3159380555152893, -0.10824967920780182, -0.04872310906648636, 0.19715555012226105, -0.2658633291721344, -0.06968845427036285, 0.009916169568896294, 0.18593478202819824, -0.038871243596076965, -0.3416462540626526, 0.1855567842721939, 0.21629339456558228, -0.10832708328962326, -0.04190235957503319, 0.2388715296983719, -0.11624565720558167, -0.10361404716968536, 0.0536813959479332, 0.12528158724308014, -0.262010782957077, 0.05081893876194954, 0.29551735520362854, 0.05958620831370354, -0.01989975944161415, -0.19261345267295837, 0.01736867055296898, -0.07923264801502228, -0.4404444694519043, 0.3125889301300049, 0.10095971822738647, 0.17173698544502258, 0.23782190680503845, -0.07170403748750687, 0.013639729470014572, 0.19007621705532074, 0.1901141107082367, -0.052342064678668976, -0.9643150568008423, -0.12307217717170715, -0.21010802686214447, -0.5640560984611511, 0.010125457309186459, 0.1314179003238678, 0.10721258819103241, -0.24371789395809174, -0.5925355553627014, 0.49424877762794495, -0.03528435528278351, -0.21386614441871643, 1.4134130477905273, -0.2751445770263672, 0.007012579124420881, -0.023824317380785942, 0.004113825503736734, -0.06332013010978699, 0.286077082157135, 0.04896686226129532, 0.31404414772987366, 0.15028351545333862, 0.003490754636004567, 0.0802399218082428, -0.230818971991539, -0.022719932720065117, 0.26083019375801086, -0.2885863184928894, 0.07537354528903961, 0.12282905727624893, -0.38638314604759216, 0.1752759963274002, -0.07370781153440475, 0.13994526863098145, 0.13313405215740204, 0.2851952016353607, 0.905279278755188, 0.34521353244781494, -0.36453402042388916, 0.46360254287719727, -0.002040385501459241, -0.003476516343653202, -0.19058215618133545, 0.27096763253211975, 0.08722586184740067, 0.03202880546450615, -0.06164764240384102, 0.011678489856421947, 0.21189850568771362, -0.40100231766700745, 0.022941868752241135, 0.0394427627325058, 0.0675845518708229, -0.22503064572811127, 0.14730903506278992, 0.24842065572738647, -0.34360530972480774, 0.21811245381832123, -0.05238509923219681, -0.008763357996940613, -0.1336073875427246, 0.15671105682849884, 0.4475333094596863, -0.5187726616859436, 0.005388418212532997, -0.07889139652252197, 0.10729073733091354, 0.22381159663200378, 0.07434546202421188, -0.0843898206949234, 0.13574494421482086, 0.01853088103234768, -0.41072791814804077, 0.40448933839797974, -0.8231801986694336, -0.4780847728252411, -0.11237931996583939, 0.012673617340624332, -0.04672158136963844, -0.23933981359004974, 0.01667654886841774, -0.14681674540042877, 0.077765092253685, 0.15309257805347443, 0.03254099190235138, -0.015896232798695564, -0.029608771204948425, -0.288953959941864, -0.32651081681251526, 0.06307528167963028, -0.09873636066913605, 0.08938323706388474, 0.27018269896507263, 0.018129458650946617, -0.050469521433115005, -0.17951229214668274, 0.02319747768342495, 0.06737810373306274, 0.2690926194190979, -0.10778623819351196, -0.04740763455629349, 0.30407941341400146, -0.08746829628944397, -0.2184152454137802, 0.14826175570487976, 0.18092381954193115, 0.07989493757486343, -0.1297195851802826],
                b1: &[-0.5191351175308228, 0.6662623882293701, 0.610133707523346, -1.1585999727249146, 0.6903770565986633, 0.4241520166397095, 0.754120945930481, -0.7599878907203674, -0.3445088267326355, 0.9317805767059326, -0.2041703462600708, 0.17219330370426178, 1.1566059589385986, -0.41121166944503784, -0.6977726817131042, 0.7911778092384338, 0.6611397862434387, -0.6938921213150024, -0.03742314130067825, -0.16022440791130066, 0.11257349699735641, 0.07743008434772491, -0.6286312937736511, 0.544836699962616, -0.15634237229824066, -0.5572881698608398, 0.9681645035743713, -0.7440500855445862, 0.10288882255554199, 0.9043763875961304, 0.14654643833637238, -0.024421239271759987, -0.4609592854976654, 0.917902410030365, 0.2704138457775116, 0.6341348886489868, 0.034945350140333176, 0.5565919876098633, 0.1746397614479065, -0.6341800093650818],
                w2: &[0.07229708135128021, 0.2507615387439728, 0.16330942511558533, 0.5204483866691589, 0.24313874542713165, -0.5474504232406616, -0.28332123160362244, -0.2225571572780609, -0.1043124571442604, 0.06595291197299957, 0.21239061653614044, -0.14725270867347717, -0.8134568333625793, 0.07381946593523026, -0.24956485629081726, 0.4919748604297638, 0.2962062954902649, 0.3260444402694702, 0.07504145801067352, -0.053836897015571594, 0.2531750500202179, -0.04855559393763542, -0.5578967332839966, -0.5225025415420532, 0.055111128836870193, -0.21510563790798187, 0.5871708989143372, -0.19132649898529053, 0.007392226252704859, -0.298953115940094, 0.16707110404968262, -0.04706822335720062, 0.07302752882242203, -0.08172990381717682, 0.23955324292182922, -0.15824700891971588, -0.3977665305137634, 0.5267415642738342, -0.11258449405431747, -0.3343915045261383, 0.23245088756084442, -0.7491211891174316, -0.6333310604095459, 0.0232061930000782, -0.2315434217453003, -0.3745144307613373, -0.03209906071424484, -0.4041699469089508, 0.041345734149217606, 0.19181972742080688, -0.2760458290576935, -0.07779327034950256, 0.24569696187973022, -0.18802686035633087, -0.6544056534767151, 0.556419849395752, 0.11468080431222916, -0.32528090476989746, 0.38538315892219543, 0.33702555298805237, -0.442532479763031, 0.00750756124034524, -0.45737770199775696, -0.06860284507274628, -0.4411284625530243, -0.23914210498332977, 0.06834587454795837, 0.14571186900138855, 0.6887655258178711, 0.5702284574508667, 0.3135473430156708, -0.3360161781311035, -0.5353860259056091, 0.06292688101530075, 0.735708475112915, 0.7143703103065491, -0.3693147897720337, 0.525284469127655, 0.39448651671409607, -0.09941494464874268, 0.09564384818077087, 0.5881519913673401, 0.05619557946920395, 0.4508857727050781, -0.2834583520889282, -0.16902177035808563, 0.24799591302871704, -0.182522252202034, 0.0468696765601635, 0.14808374643325806, -0.013205822557210922, -0.12705814838409424, 0.0614711195230484, 0.14103399217128754, -0.2599405348300934, 0.028414186090230942, -0.2865449786186218, -0.08163938671350479, 0.13120926916599274, 0.17990124225616455, -0.16350798308849335, -0.09809352457523346, -0.013590727932751179, -0.17736633121967316, 0.05107983574271202, 0.3411618173122406, -0.2772451341152191, 0.32397109270095825, 0.046551186591386795, 0.13246433436870575, 0.05053735896945, 0.24057962000370026, -0.04693610221147537, -0.1650579869747162, 0.1331019252538681, 0.09457181394100189, -0.16547952592372894, -0.09469929337501526, 0.30049434304237366, 0.12664170563220978, -0.013082812540233135, 0.390655517578125, 0.6400918364524841, -0.0010483618825674057, -0.03533017635345459, 0.16345657408237457, 0.05697643384337425, 0.1748565286397934, 0.0036667422391474247, -0.05557025969028473, 0.016822226345539093, -0.12541711330413818, -0.4695605933666229, 0.008447905071079731, 0.16371716558933258, -0.1481284201145172, -0.10916673392057419, 0.1754710078239441, -0.05557332932949066, 0.17406205832958221, 0.03734235838055611, -0.0014076621737331152, 0.16409075260162354, -0.0339696928858757, 0.11525241285562515, 0.11995170265436172, -0.39020177721977234, 0.01936984248459339, -0.14390763640403748, -0.18344464898109436, -0.08675119280815125, 0.19569827616214752, 0.48439380526542664, -0.232485830783844, -0.004231136757880449, 0.15202505886554718, 0.01103641465306282, -0.1192987710237503, -0.17487019300460815, 0.27336806058883667, -0.5894135236740112, -0.03331466019153595, 0.21942859888076782, 0.30420297384262085, 0.2666693329811096, 0.4481956958770752, -0.020630693063139915, 0.8494743704795837, 0.5691520571708679, 0.5711295008659363, 0.00404204148799181, 0.5070351958274841, 0.09074786305427551, 0.15874768793582916, 0.7676622271537781, 0.6556511521339417, 0.1220490038394928, 0.7263025641441345, -0.07173441350460052, 0.14413252472877502, 0.49090006947517395, -0.3324028253555298, 0.45898303389549255, 0.5931536555290222, 0.19021296501159668, -0.7473744750022888, -0.834629476070404, -0.1385311633348465, -0.05174582824110985, 0.018871335312724113, -0.42817312479019165, 0.20682017505168915, 0.016382897272706032, -0.6684255599975586, 0.3525462746620178, -0.42306870222091675, -0.0817568302154541, 0.3572525084018707, -0.23954586684703827, -0.4869120717048645, 0.016070470213890076, 0.5639761686325073, 0.17797298729419708, 0.2919785678386688, -0.3837592601776123, 0.13362792134284973, 0.09925093501806259, 0.12642522156238556, 0.09690988808870316, -0.08732952922582626, 0.24605968594551086, -0.3894798457622528, -0.174991175532341, 0.2573908269405365, 0.22514064610004425, -0.24535547196865082, -0.2993263006210327, 0.24350187182426453, 0.03375721350312233, 0.16244018077850342, -0.16753582656383514, -0.08621060848236084, 0.1272309273481369, 0.007472787983715534, 0.20557984709739685, 0.1578531116247177, -0.5838948488235474, 0.08410368114709854, -0.2831973135471344, -0.28126293420791626, -0.08023717254400253, 0.5180243849754333, 0.2208152413368225, -0.3613019585609436, -0.06204051896929741, -0.13526616990566254, 0.09384715557098389, -0.27185022830963135, -0.05938927084207535, 0.284194678068161, 0.04228530079126358, 0.5006632208824158, 0.6578063368797302, -0.07014274597167969, -0.3233219087123871, -0.01618030108511448, 0.2888641357421875, -0.08185673505067825, -0.17689819633960724, -0.2994365096092224, 0.016244128346443176, 0.02359011210501194, 0.1367129534482956, -0.01653127372264862, -0.09157261997461319, -0.3516620397567749, -0.09030301123857498, -0.07817772775888443, 0.17603041231632233, -0.01393663790076971, -0.029468189924955368, -0.0814921036362648, -0.12077502906322479, -0.10759524255990982, -0.0750858411192894, 0.2511105239391327, -0.20753242075443268, -0.05136517807841301, -0.024205535650253296, -0.3384825587272644, 0.020664114505052567, 0.11200296878814697, 0.08333364874124527, -0.24177855253219604, -0.07010341435670853, 0.020779477432370186, -0.20839253067970276, -0.0016562794335186481, 0.023504814133048058, 0.3570723235607147, -0.30022287368774414, -0.3554439842700958, -0.027536675333976746, -1.1282703876495361, -0.08706718683242798, 0.0742080882191658, 0.18080361187458038, -0.02274167723953724, -0.704075813293457, -0.9722687602043152, 0.1188407614827156, -0.029379399493336678, 0.8019110560417175, -0.34810709953308105, 0.04902748018503189, -0.7494327425956726, 0.5064789056777954, -0.11681736260652542, 0.2257058471441269, -0.4354608356952667, 0.3252757489681244, -0.1591869592666626, -0.5933760404586792, -0.5259361863136292, 0.22252318263053894, 0.30712220072746277, 0.29186123609542847, -0.7899709343910217, 0.3455640971660614, -0.8577526807785034, 0.19282177090644836, 0.29095181822776794, -0.3287593424320221, 0.0454283282160759, -0.5983009338378906, -0.08342050760984421, -0.8976981043815613, 0.10165920853614807, 0.13396088778972626, 0.2290259599685669, 0.02499830722808838, 0.7539560794830322, 0.1477266401052475, 0.3097168207168579, -0.3993585705757141, 0.0817292109131813, 0.038499560207128525, 0.048502497375011444, 0.10572300106287003, -0.17650842666625977, 0.30300378799438477, -0.3586488962173462, -0.09699319303035736, 0.28980425000190735, 0.1152607873082161, -0.30993735790252686, -0.3226162791252136, 0.2082981914281845, 0.08206543326377869, 0.09643732011318207, -0.09098457545042038, -0.09191355854272842, 0.04240717366337776, -0.08706614375114441, 0.3119218051433563, 0.24132680892944336, -0.5137639045715332, 0.03463784605264664, -0.29585450887680054, -0.3583862781524658, -0.09919128566980362, 0.5263358950614929, 0.19875890016555786, -0.4007430374622345, -0.044145308434963226, -0.24342355132102966, 0.16471655666828156, -0.25901785492897034, 0.012997856363654137, 0.3298455476760864, -0.23130790889263153, 0.4484388828277588, 0.35633817315101624, 0.26454973220825195, 0.15214529633522034, -0.12443697452545166, -0.405061811208725, 0.17236965894699097, -0.36522531509399414, -0.074102483689785, 0.09564346820116043, -0.26696014404296875, -0.7053405046463013, -0.4750596880912781, 0.2850874066352844, -0.42413032054901123, 0.3273111581802368, 0.013779409229755402, -0.7248923182487488, -0.49210208654403687, 0.5041399002075195, -0.14308881759643555, 0.629442036151886, -0.8470776677131653, 0.36798736453056335, -0.17092065513134003, 0.5437707304954529, -0.26034078001976013, -0.4502609074115753, 0.2898317873477936, -0.3266198933124542, 0.1681036651134491, 0.6064534783363342, 0.48974573612213135, -0.3461318910121918, -0.36192092299461365, 0.3675844371318817, -0.731248676776886, -0.21227769553661346, -0.4246974289417267, 0.17397946119308472, -0.3643985986709595, 0.205714613199234, 0.629838228225708, 0.10543780773878098, 0.010421440936625004, 0.6487590670585632, -0.685522198677063, 0.010746597312390804, 0.371294766664505, -0.68584144115448, 0.69797283411026, -0.39890381693840027, 0.2957388460636139, 0.10036955028772354, -0.31620606780052185, -0.5876231789588928, -0.5783882737159729, -0.4745366871356964, 0.20689401030540466, -0.2748165428638458, 0.34110450744628906, 0.817054033279419, 0.8686729073524475, -0.6139298677444458, -0.19506172835826874, -0.03448706120252609, 0.635860025882721, -0.38243091106414795, 0.8843176960945129, 0.08922040462493896, -0.8030375242233276, 0.01003911904990673, 0.49227485060691833, 0.02043282799422741, -0.1812848448753357, 0.8425045609474182, -0.18937410414218903, 0.2360723465681076, -0.0486280657351017, 0.1306903064250946, 0.44811540842056274, -0.09772484004497528, 0.3676001727581024, -0.10864408314228058, 0.10239739716053009, 0.26535993814468384, -0.19465096294879913, -0.05268852412700653, 0.013907784596085548, 0.11859709769487381, -0.008244873955845833, -0.12678827345371246, 0.16795198619365692, 0.09826375544071198, -0.13783332705497742, -0.32474759221076965, -0.018496913835406303, -0.12179988622665405, 0.22411927580833435, -0.10514824092388153, 0.038778163492679596, 0.33486974239349365, 0.31644245982170105, 0.05365574359893799, 0.24912847578525543, -0.31889432668685913, 0.24240325391292572, -0.19231560826301575, 0.18558776378631592, -0.022984078153967857, 0.11608095467090607, 0.15418484807014465, -0.14139854907989502, 0.01758008636534214, -0.12027571350336075, 0.2522386610507965, -0.2922046184539795, 0.049236513674259186, 0.19894357025623322, 0.39957553148269653, 0.3346879780292511, 0.3187335133552551, 0.4501717686653137, -0.8946970701217651, 0.18189306557178497, -0.08766483515501022, 0.2782788574695587, 0.3587392270565033, -0.33824455738067627, 0.6033147573471069, -0.6243746876716614, -0.6177958250045776, 0.6629742383956909, 0.4856598377227783, -0.3099081814289093, -0.678487241268158, 0.47894829511642456, -0.03139176964759827, 0.16848357021808624, -0.5739434957504272, -0.16708984971046448, 0.11146949231624603, 0.090438611805439, 0.4812713861465454, 0.5129365921020508, -0.7324693202972412, 0.26365718245506287, -0.4824923276901245, -0.5487518310546875, -0.20128659904003143, 0.5759150385856628, 0.3504473567008972, -0.36605504155158997, -0.4257725477218628, -0.25298258662223816, 0.512897789478302, -0.4181336462497711, -0.516604483127594, 0.37244912981987],
                b2: &[0.14859354496002197, -0.018167857080698013, -0.3407953083515167, -0.14991576969623566, 0.4018653333187103, -0.2384500652551651, -0.4047893285751343, 0.15702210366725922, -0.3152092695236206, 0.29297566413879395, 0.26403820514678955, -0.2573520541191101, -0.11290331929922104],
            }
        }
