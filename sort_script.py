l = [
(1,'aalangatti mazha'),
(2,'aaluma dolum'),
(3,'aanandha yaazhai'),
(4,'aathichoodi vijay antony'),
(5,'aetho saigirai'),
(6,'ale ale'),
(7,'allah ninaithale inikkum'),
(8,'amali thumali'),
(9,'anargazhi kangalal kaithu sei'),
(10,'anbil avan'),
(11,'anjali anjali pushpanjali'),
(12,'annakili 4 students'),
(13,'arjunar villu gilli'),
(14,'aruva meesai dhool'),
(15,'athiradi karan sivaji'),
(16,'azhagiya laila'),
(17,'chennai senthamizh'),
(18,'chikkini chameli'),
(19,'chinna chinna mazhai thuligal'),
(20,'chinna chinnathai penne'),
(21,'chudithar aninthu vantha sorgame'),
(22,'dailamo dailamo'),
(23,'devathaye vaa malaikottai'),
(24,'dharshana hridayam'),
(25,'dippam dappam'),
(26,'dolu maachi deepavali'),
(27,'en anbe sathyam'),
(28,'en jannal nilavukku ennaachu'),
(29,'en jannalil therivathu mozhi'),
(30,'en kadhal solla'),
(31,'en nanbane mangatha'),
(32,'enge en punnagai'),
(33,'ennamo edho ko'),
(34,'enthan nenjil yesudhas'),
(35,'feel my love kutty'),
(36,'gulmuhar malare'),
(37,'hey baby aegan'),
(38,'hey baby raja rani'),
(39,'hey keechu kiliye mugavari'),
(40,'high on love'),
(41,'i like you citizen'),
(42,'idhu varai'),
(43,'jeevamshamyi'),
(44,'kaala chashma'),
(45,'kaatrin mozhi male'),
(46,'kadhal mattum purivathillai'),
(47,'kanaa kandaenadi'),
(48,'kannai kasakkum sooriyano red'),
(49,'kannal pesum penne'),
(50,'kannamma ispade'),
(51,'kannamma kaadhelennum'),
(52,'kannan varum'),
(53,'kannodu kanbathellam jeans'),
(54,'kannum kannum nokia'),
(55,'karu karu vizhigalal'),
(56,'kodai kaala kaatre ilayaraja'),
(57,'kunkuma nira sooryan'),
(58,'kuruvi adicha music'),
(59,'lajjavathiye'),
(60,'life of ram'),
(61,'machan peru madurey'),
(62,'malai kaatru tamil pesutho'),
(63,'paatu thalaivan paadinal'),
(64,'kutty pattas'),
(65,'malare mounama'),
(66,'manase ila manase puthiya geethai'),
(67,'mannippaaya'),
(68,'mascara pottu'),
(69,'mazhai nindra raman'),
(70,'meesaya murukku title song'),
(71,'mercury poove'),
(72,'merke merke'),
(73,'minsaram en meethu'),
(74,'mudhal mazhai bheema'),
(75,'mudhan mudalil paarthen hariharan aaha'),
(76,'mudhar kanave'),
(77,'mun paniya'),
(78,'naattu koothu'),
(79,'narumugaye'),
(80,'nee himamazhayai'),
(81,'nee kidaithai'),
(82,'nee marlyn monroe'),
(83,'nee otha sollu sollu'),
(84,'nimirndhu nil saroja'),
(85,'oh baby yaaradi nee mohiney'),
(86,'oh nenje nenje mugavari'),
(87,'oh oh sanam dasavatharam'),
(88,'oru deivam thantha poove male'),
(89,'oru paarvayil poo koduthai siva manasula sakthi'),
(90,'oru vaarthai kekka iyya'),
(91,'ottiyanam senju tharen variya'),
(92,'oxygen lyric video kavan'),
(93,'paartha mudhal naale'),
(94,'paathagathi kannupattu'),
(95,'paatu solli paada solli'),
(96,'pachai kaatre abiyum naanum'),
(97,'padichu paathen'),
(98,'paravaye engu irukkirai'),
(99,'pazha muthir solai yesudhas'),
(100,'po po yen sid sriram'),
(101,'pogathe yuvan'),
(102,'pon magal vanthal remix'),
(103,'poosum manjal male'),
(104,'poova poove poovellam kettupar'),
(105,'poove sempoove'),
(106,'ragasiyamai dum dum dum'),
(107,'roja poo aadivanthathu'),
(108,'saayndhu saaynthu'),
(109,'sada sada kavalan'),
(110,'samajavaragamana'),
(111,'sangeetha megam'),
(112,'seval kozhi billa'),
(113,'sevvaanam selai katti'),
(114,'so baby doctor'),
(115,'swasame thenali'),
(116,'taj mahal oviya kadhal'),
(117,'thaakuthe kan thaakuthe'),
(118,'thani oruvan ninaithu vittal'),
(119,'thanni karuthidichu'),
(120,'thanthana thanthana thavasi'),
(121,'thean thean thean kuruvi'),
(122,'thirunelveli halwa da'),
(123,'thodu thodu enave'),
(124,'thom karuvil irunthom'),
(125,'thozhiya en kadhaliya'),
(126,'top tucker sarkar rahman tamil'),
(127,'ulaga nayagane dasavatharam'),
(128,'un paarvai mele pattal'),
(129,'unakkena naan enakkena nee'),
(130,'unakkul naane bombay jayashree'),
(131,'unthan desathin'),
(132,'uyirin uyire kaaka kaaka'),
(133,'vaa vaa en devathaye aabiyum naanum'),
(134,'vaadi pulla vaadi'),
(135,'vaaji vaaji'),
(136,'vachan vachan silambattam'),
(137,'vaseegara'),
(138,'verenna verenna vendum'),
(139,'vikram title track'),
(140,'vilambara idaivelai'),
(141,'yaakai thiri'),
(142,'yaarodu yaaro yogi'),
(143,'yeh nilave mugavari'),
(144, 'just the two of us'),
(145, 'manjal veyil'),
(146, 'parayuvan malayalam'),
(147, 'ragatulla'),
(148, 'na na na mervin'),
(149, 'nenje nenje ayan'),
(150, 'nenje nenje ratchagan'),
(151, 'anbe idhu rhythm'),
(152, 'ayyo pathikichu'),
(153, 'poovukku poranthanalu little john'),
(154, 'yaar aval yaaro muppozhuthum un karpanaigal')
]

l = [i[1] for i in l]
l.sort()
for i in range(1, len(l)+1):
    print(f"({i}, '{l[i-1]}')", end="")
    if i!=len(l):
        print(",")
    else:
        print(";")