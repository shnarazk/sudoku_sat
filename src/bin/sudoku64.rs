use {
    miracle_sudoku::{cell::*, get_range, pos::*, set_range, sudoku::*, Rules},
    splr::*,
    std::{collections::HashMap, convert::TryFrom, fs::File, io::Write},
};

pub fn main() {
    let range = 64;
    set_range(64);
    assert_eq!(get_range(), range);
    let mut rules: Rules = Vec::new();
    let (conf, mut dic) = parse_sudoku();
    rules.append(&mut sudoku_ident(&conf));
    rules.append(&mut sudoku_ident2());
    rules.append(&mut sudoku_row(&conf));
    rules.append(&mut sudoku_column(&conf));
    rules.append(&mut sudoku_block(&conf));
    rules.append(&mut sudoku_preset(&conf));
    // csv_sudoku();
    let setting: Vec<i32> = conf
        .iter()
        .map(|(p, d)| p.state(*d, true).as_lit())
        .collect::<Vec<_>>();
    let mut file = File::create("sudoku64.cnf").expect("fail to create 'sudoku64.cnf'");
    file.write_all(&miracle_sudoku::cnf::as_cnf_u8(&rules, &[]))
        .expect("fail to write 'sudoku64.cnf'");
    println!("#rules: {}", rules.len());
    let config = splr::Config {
        splr_interface: true,
        quiet_mode: false,
        ..Default::default()
    };
    let mut solver = Solver::try_from((config, rules.as_ref())).expect("panic");
    for a in setting.iter() {
        solver.add_assignment(*a).expect("panic");
    }
    println!("running...");
    let mut answer: Vec<Vec<usize>> = Vec::new();
    for ans in solver.iter().take(1) {
        println!("found!");
        let mut picked = ans.iter().filter(|l| 0 < **l).collect::<Vec<&i32>>();
        // println!("{}: {:?}", ans.len(), picked);
        assert_eq!((range * range) as usize, picked.len());
        for _i in 1..=range {
            let mut line: Vec<usize> = Vec::new();
            for _j in 1..=range {
                let (_i, _j, d, _b) = Cell::decode(*picked.remove(0));
                line.push(d);
                print!("{}", *dic.entry(d).or_default());
            }
            answer.push(line);
            println!();
        }
        println!();
    }
    println!("verified {}", verify(&answer));
}

const SUDOKU: &str = "\
F   aAr z$yGQ    6   eK  4 1sT   BER pjcN  JqZ d   S f2   5o    
Vf m  Di u T    qjvIU O8E   c   F  H  KQz  ys  P+9p3#  5  6   J 
  U1  QY Ecd tD fCs $hLkwe   3  imq  J M   S  n  8 6 H  Z 4 F yr
E O vnj  a p H1  A79Pz+  F o2U r3  Y g CM   I     NG ?et  x8R iX
   xSHJ  R9?CAN i dnaQ   M IgX G    4  y   hU  #EwjzT   vqf  beY
2 L9hwy# UB  PZ     ? m    a 568+xb7 X S3FijQ   c     I T N M V 
 bzI GP    e  S J Z  3roiy   xOfv  Vt2 a6m wH  7    C $ dj k  c 
     3  8I   6WY    c u   JR ? 7n     5 1Kv$BxaD i AF rMQ PgwECU
  v   z A #uTC w XxPo W a9      UR?kLBtg  5 b  Yh 1ES   y d  M  
J EV2  si j    1+gzZM   FHx  eBw  y Tf #K pX   c7 m nuNbCR      
G  UHr5a  S e  hcs JFjw  iVY4 zKO        xmk   T   oWX9B  vqIpn2
Mxi  Rq    E f o  Btr  Hc$Z h   4 7pb   L+ COy n 5KYPT  WuXs G j
  uk   A      RX Q  #b qCgm M  +  oiIV Ke7Ur  3W 4vx dHwYP   O B
 9oO C BaxzHZy M p Uf  i j  1N R83S  n mw  ?Vd4 AL#g6    FKc+5  
 W+     d q b   nD TCS   PL o   z6Jcr h29 f A 1syM?    jU 74k$ x
T Z  hX3  L 4 vQV ONGk  2nt  b  $P  YH F  j     If  cqJ   9?  # 
fr S9 $ 6 + L2 B34 QWyc   #?KE  N7  PZX  RoU  Ybt    Mwnmig Dh  
3 n   mDW ijdZ  8I Y p26  AxzyM V$+ C S    s c5   X qB 7aHbGrw 4
k  u x4I   PsGhN r?R       q Wt  v Fje   AT6   Jf gCm bi  EQ  o 
H  Q WYoS kt wzc   g1X s6 v  P Dh8d    ? iCxpajm#A  3  RL JMf  y
Ag8N B ybp3  uaD Tn  7CU   rd G5 Y   O   vS4wqQ21 P  K? I Z   WF
5XK cV  y8H m  UwELzq    b+jNs 4 Ak  tG  u?n  Z TQ dYW S  3 # v 
p?M     rfg      Ot ba#9V I TFYXDJBnx5i W1+8 h  NcG     A 2j sP 
R  i+ dGJ ?A 9   $h   D  Z m7 Sp#sa  o 6 B fX Mg8r vIk eO nKVN 1
  jX pC UbE4   2W     o D5T mJ #  Lv7 z     t3 i?1 eQnc f $ N 6Z
W4 GENV       t  3Hu dAD+IM$ n k  hCc?R      Lzx   T7 Uvo r  Y5 
wA  8 tF  m M C E ye+GBR  U  v  Yk6$ q  b 2  puV   W r          
 if2L  q  u1 gc t      CyX?E       OW  3hY  v8 4   K$ o   a   D 
U Py6QZ  d I3  T$  2   pH        9VJu NX? W  Rm    h  G bw vjxEO
 Omcb gJ XQx  8 6ZN j9Fa   7 qC 2EIrf +oA DM s k   Lt S h   ?B d
   ?  1  j v LK$S8b  g  RA   Z h  Q   D Xn P9G O2  mz x  eT 4  p
 d s$   e  o +P #   kxT4z  b  ra  i G      Q6w N9u DH EO  tX  m 
OPXZpTH #7MYD  nGKr     s   3  ? b4ty  RJw q Ndvz x E 5$  Qm aS9
hkAY f  $P8sK mEd    WQt#1iDw 4F     r  poO  uc?bgnqG      R H 5
    z n?  v3    u   ew1Ao 9 + TbS   a   PshDKF  RC  X p    x    
   5 b    w   qA mo#H SX pcPt2ZEQ  B UCW4  Yk f Fs +r  D $MIuy g
+c  FSG uy 9r p5Mn2CVLkJ    X  x h  HP   Q7 mW$AoY  v  4tZ Tqd w
d6 J       O   tz       $    Ah gTs+qvcfZH1 RbB K 9y2   e oY ?pk
C   sm  4hG jRBWy   Iq3 vN7 fOgn  XKi  et    T E    A   z  r #F 
DU   q9   oc 1  TF   Oh  GWS m  xIM?nE6u5L#  z  e   ki  K8 J   +
   Am 8Ksk  $I   y  z nP   iZ   j  T5xe SE BfV   NRbowW     OJMD
N $b yi D R  7  ?c d  Zuj X Q   pa #VwF8 6   PH v s  E  BUIeAgTm
  g4 E  Zcp   V Hx A   B S     N   D6Y 7   2?KReMXua5#ky  1$ 9  
 L1a D+M? Ario6    kv  jpuzsI 8  S  K  dqO   #y$x  P hC 43     W
  Yn 9Rr B  fUj  b  L  51dD VCq      3QH h    W  ?SZJ2g w7#aXztc
q ?    p 9Nn 3e#XYKmgr O x cUR   o 4Z+b   8 DQv  TB  I     2H  C
sZwe Uvu 1WbXY  ha  28VS  Bf?KJ I g zm    9+GrA  td4LQ  6      i
#zBh 7     F x H4eipso 3g25LPMk  O 9  frm d  IJu n+  $ cV      S
 1DHZiBjQ3  #m Lp Y hJ  bOoU  I 5? N    2 z r  w   u t+ 7 c x6 8
L3 w ?O  5 WU$ R2 gX9   hm1  YABKd u 7 n kc Z   ipC szVEP4 HJ a#
 tQqR s2   C Xbf zVy    n?G i   rwO    x78  oB LmKJ#MjhP1W  $  3
$MV  P   t 8uig R C 3N? eT kWja AZ   s  + yO1m#pn Fr   X 5  vS G
  y dvbT r  BjxKLPE5wM71 3R     9t m     q  W4 G  k gAZfpOun2CYo
 GSCrz U2vOMp      c T  N8y     B4  +$Vj bn eitQ  HR   L A    q 
n 9  F xVY  E?wyvoI4   Z  u  6f J  X2cPbgS3 5j  Q D  G 1rNB d   
oEN+   f Z   F    G O  W s$w td yM#hg qzRV uPH CS Y Bb3? K  LU  
 sa   f9cK   D#   $ m d L  n        F  UQ2x zSrjC A + ig  k  o v
Iu3 #O7b  Z     gfFqJBtQ  P6  X   jzpRs o  9 +      ac4 8V 5Yr H
   LQ   hT wWNFs  6j7H9MKtg5R eYZrn J # d Pa     $V     ?p ycX  
 2 rT  V   m     WP Y+pz  S G i96 t EM8 F 4K C fwRO   qJx   QLA7
   d  WCY   o H9   K85XxTaO2y    qG m  I$ A  1D  P6tu Q  S  b   
1J g                   ?ro MF u  yH DL +i#    V ZG72p       3R n
z8  iohmgCn+6M 4 k V URI?   pB W   Pw T O uv  G d b f3FNJa  K   
cv K  E       Q   u    eIzf   1   W S O y Y3 kb  B H s   6G9UmjN";

fn parse_sudoku() -> (Vec<(Pos, usize)>, HashMap<usize, char>) {
    let mut dic: HashMap<char, usize> = HashMap::new();
    let mut used: HashMap<char, bool> = HashMap::new();
    let mut val = 1;
    for c in 'A'..='Z' {
        dic.insert(c, val);
        used.insert(c, false);
        val += 1;
    }
    for c in 'a'..='z' {
        if c == 'l' {
            continue;
        }
        dic.insert(c, val);
        used.insert(c, false);
        val += 1;
    }
    for c in '1'..='9' {
        dic.insert(c, val);
        used.insert(c, false);
        val += 1;
    }
    dic.insert('#', val);
    used.insert('#', false);
    val += 1;
    dic.insert('?', val);
    used.insert('?', false);
    val += 1;
    dic.insert('+', val);
    used.insert('+', false);
    val += 1;
    dic.insert('$', val);
    used.insert('$', false);
    assert_eq!(val, 64);

    let mut vec: Vec<(Pos, usize)> = Vec::new();

    for (i, l) in SUDOKU.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            let p = Pos::at(1 + i as isize, 1 + j as isize);
            if let Some(code) = dic.get(&c) {
                vec.push((p, *code));
                *used.entry(c).or_insert(false) = true;
            } else {
                panic!("{} not found", c);
            }
        }
    }
    let mut pull_back: HashMap<usize, char> = HashMap::new();
    for e in &used {
        if *e.1 {
            pull_back.insert(*dic.entry(*e.0).or_insert(0), *e.0);
        } else {
            println!("{:?}", e);
        }
    }
    (vec, pull_back)
}

#[allow(dead_code)]
fn csv_sudoku() {
    for l in SUDOKU.lines() {
        for c in l.chars() {
            print!("'{}',", c);
        }
        println!();
    }
}
