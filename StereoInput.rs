ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAAm;
				li:objects {
					ha:group.1 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAn; loclib_name=lm393_so8;
						li:objects {
						}
						ha:attrib {
							footprint=so(8)
							li:portmap {
								{1/in- -> pcb/pinnum=2}
								{1/in+ -> pcb/pinnum=3}
								{1/out -> pcb/pinnum=1}
								{1/V+ -> pcb/pinnum=8}
								{1/V- -> pcb/pinnum=4}
								{2/in- -> pcb/pinnum=6}
								{2/in+ -> pcb/pinnum=5}
								{2/out -> pcb/pinnum=7}
								{2/V+ -> pcb/pinnum=8}
								{2/V- -> pcb/pinnum=4}
							}
						}
					}
					ha:group.2 {
						uuid=fMIO8VR/GToe4JUvNI8AAABo; loclib_name=quad_opamp_so14;
						li:objects {
						}
						ha:attrib {
							footprint=so(14)
							li:portmap {
								{1/out -> pcb/pinnum=1}
								{1/in- -> pcb/pinnum=2}
								{1/in+ -> pcb/pinnum=3}
								{1/V+ -> pcb/pinnum=4}
								{1/V- -> pcb/pinnum=11}
								{2/out -> pcb/pinnum=7}
								{2/in- -> pcb/pinnum=6}
								{2/in+ -> pcb/pinnum=5}
								{2/V+ -> pcb/pinnum=4}
								{2/V- -> pcb/pinnum=11}
								{3/out -> pcb/pinnum=8}
								{3/in- -> pcb/pinnum=9}
								{3/in+ -> pcb/pinnum=10}
								{3/V+ -> pcb/pinnum=4}
								{3/V- -> pcb/pinnum=11}
								{4/out -> pcb/pinnum=14}
								{4/in- -> pcb/pinnum=13}
								{4/in+ -> pcb/pinnum=12}
								{4/V+ -> pcb/pinnum=4}
								{4/V- -> pcb/pinnum=11}
							}
						}
					}
					ha:group.3 {
						uuid=fMIO8VR/GToe4JUvNI8AAABp; loclib_name=quad_opamp_so14;
						li:objects {
						}
						ha:attrib {
							footprint=so(14)
							li:portmap {
								{1/out -> pcb/pinnum=1}
								{1/in- -> pcb/pinnum=2}
								{1/in+ -> pcb/pinnum=3}
								{1/V+ -> pcb/pinnum=4}
								{1/V- -> pcb/pinnum=11}
								{2/out -> pcb/pinnum=7}
								{2/in- -> pcb/pinnum=6}
								{2/in+ -> pcb/pinnum=5}
								{2/V+ -> pcb/pinnum=4}
								{2/V- -> pcb/pinnum=11}
								{3/out -> pcb/pinnum=8}
								{3/in- -> pcb/pinnum=9}
								{3/in+ -> pcb/pinnum=10}
								{3/V+ -> pcb/pinnum=4}
								{3/V- -> pcb/pinnum=11}
								{4/out -> pcb/pinnum=14}
								{4/in- -> pcb/pinnum=13}
								{4/in+ -> pcb/pinnum=12}
								{4/V+ -> pcb/pinnum=4}
								{4/V- -> pcb/pinnum=11}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=ZZ4sFFZV2GbC8tlDpjoAAAAE;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.1 {
				uuid=8/SYZ/pfBzxyqFA12WQAAAAC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAg;
				li:objects {
					ha:polygon.11 {
						li:outline {
							ha:line { x1=0; y1=0; x2=80000; y2=0; }
							ha:line { x1=80000; y1=0; x2=80000; y2=20000; }
							ha:line { x1=80000; y1=20000; x2=0; y2=20000; }
							ha:line { x1=0; y1=20000; x2=0; y2=0; }
						}
						stroke=titlebox-frame;
						fill=titlebox-fill;
					}
					ha:line.12 { x1=0; y1=10000; x2=80000; y2=10000; stroke=titlebox-frame; }
					ha:line.13 { x1=40000; y1=10000; x2=40000; y2=0; stroke=titlebox-frame; }
					ha:text.20 { x1=1000; y1=16500; dyntext=0; stroke=titlebox-big; text=TITLE; }
					ha:text.21 { x1=1000; y1=10500; x2=79000; y2=16000; dyntext=1; stroke=titlebox-big; text=%../../A.title%; }
					ha:text.22 { x1=1000; y1=5500; dyntext=0; stroke=titlebox-small; text={PROJECT:}; }
					ha:text.23 { x1=13000; y1=5500; x2=39000; y2=9500; dyntext=1; stroke=titlebox-big; text=%project.name%; }
					ha:text.24 { x1=1000; y1=500; dyntext=0; stroke=titlebox-small; text={PAGE:}; }
					ha:text.25 { x1=10000; y1=500; x2=39000; y2=4500; dyntext=1; stroke=titlebox-big; text=%../../A.page%; }
					ha:text.26 { x1=41000; y1=5500; dyntext=0; stroke=titlebox-small; text={FILE:}; }
					ha:text.27 { x1=48000; y1=5500; x2=79000; y2=9500; dyntext=1; stroke=titlebox-big; text=%filename%; }
					ha:text.28 { x1=41000; y1=500; dyntext=0; stroke=titlebox-small; text={MAINTAINER:}; }
					ha:text.29 { x1=55000; y1=500; x2=79000; y2=4500; dyntext=1; stroke=titlebox-big; text=%../../A.maintainer%; }
				}
				ha:attrib {
					purpose=titlebox
				}
			}
			ha:group.2 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAAO; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=164000; y=588000;
				li:objects {
					ha:group.1 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAP; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAR; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAS; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAT; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=1
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U1
					role=symbol
				}
			}
			ha:group.3 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAAU; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=164000; y=536000;
				li:objects {
					ha:group.1 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAV; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAW; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAX; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAY; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=2
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U1
					role=symbol
				}
			}
			ha:group.4 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAAa; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=276000; y=232000;
				li:objects {
					ha:group.1 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAb; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAd; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAe; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAf; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=1
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U4
					role=symbol
				}
			}
			ha:group.5 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAAg; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=276000; y=84000;
				li:objects {
					ha:group.1 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAh; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAi; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAj; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAk; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAl; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=4
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U4
					role=symbol
				}
			}
			ha:group.6 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAAu; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=88000; y=584000;
				li:objects {
					ha:group.1 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAv; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAAw; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.7 {
				uuid=FJTdh8CXI0wmXfEvXIIAAABD; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=112000; y=584000;
				li:objects {
					ha:group.1 {
						uuid=FJTdh8CXI0wmXfEvXIIAAABE; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAABF; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.8 {
				uuid=FJTdh8CXI0wmXfEvXIIAAADC; src_uuid=FJTdh8CXI0wmXfEvXIIAAAC/;
				x=56000; y=576000; mirx=1;
				li:objects {
					ha:text.1 { x1=-4000; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAADD; src_uuid=FJTdh8CXI0wmXfEvXIIAAADA;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAADE; src_uuid=FJTdh8CXI0wmXfEvXIIAAADB;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					name=CONN??
					role=symbol
				}
			}
			ha:group.9 {
				uuid=FJTdh8CXI0wmXfEvXIIAAADF; src_uuid=FJTdh8CXI0wmXfEvXIIAAAC/;
				x=56000; y=524000; mirx=1;
				li:objects {
					ha:text.1 { x1=-4000; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAADG; src_uuid=FJTdh8CXI0wmXfEvXIIAAADA;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAADH; src_uuid=FJTdh8CXI0wmXfEvXIIAAADB;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					name=CONN??
					role=symbol
				}
			}
			ha:group.10 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAHS; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHH;
				x=820000; y=792000;
				li:objects {
					ha:text.1 { x1=0; y1=-2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHT; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHI;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHU; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHJ;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHV; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHK;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHW; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHL;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHX; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHM;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHY; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHN;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:group.8 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHZ; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHO;
						x=0; y=24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=7
							role=terminal
						}
					}
					ha:group.9 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHa; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHP;
						x=0; y=28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=8
							role=terminal
						}
					}
					ha:group.10 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHb; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHQ;
						x=0; y=32000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=9
							role=terminal
						}
					}
					ha:group.11 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHc; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHR;
						x=0; y=36000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=10
							role=terminal
						}
					}
					ha:polygon.12 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=38000; }
							ha:line { x1=0; y1=38000; x2=4000; y2=38000; }
							ha:line { x1=4000; y1=38000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					name=CONN??
					role=symbol
				}
			}
			ha:group.11 {
				uuid=FJTdh8CXI0wmXfEvXIIAAAHd; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHH;
				x=380000; y=148000;
				li:objects {
					ha:text.1 { x1=0; y1=-2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHe; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHI;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHf; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHJ;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHg; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHK;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHh; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHL;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHi; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHM;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHj; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHN;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:group.8 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHk; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHO;
						x=0; y=24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=7
							role=terminal
						}
					}
					ha:group.9 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHl; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHP;
						x=0; y=28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=8
							role=terminal
						}
					}
					ha:group.10 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHm; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHQ;
						x=0; y=32000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=9
							role=terminal
						}
					}
					ha:group.11 {
						uuid=FJTdh8CXI0wmXfEvXIIAAAHn; src_uuid=FJTdh8CXI0wmXfEvXIIAAAHR;
						x=0; y=36000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=10
							role=terminal
						}
					}
					ha:polygon.12 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=38000; }
							ha:line { x1=0; y1=38000; x2=4000; y2=38000; }
							ha:line { x1=4000; y1=38000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					name=CONN??
					role=symbol
				}
			}
			ha:group.51 {
				uuid=+OS4dgTbyDjODhovSXUAAABU; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIk;
				x=84000; y=584000; rot=270.000000;
				li:objects {
					ha:arc.1 { cx=2000; cy=-12000; r=2000; sang=0.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.2 { cx=6000; cy=-12000; r=2000; sang=0.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.3 { cx=10000; cy=-12000; r=2000; sang=0.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.4 { cx=2000; cy=-4000; r=2000; sang=180.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.5 { cx=6000; cy=-4000; r=2000; sang=180.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.6 { cx=10000; cy=-4000; r=2000; sang=180.000000; dang=180.000000; stroke=sym-decor; }
					ha:line.7 { x1=0; y1=-9000; x2=12000; y2=-9000; stroke=sym-decor; }
					ha:line.8 { x1=0; y1=-7000; x2=12000; y2=-7000; stroke=sym-decor; }
					ha:group.9 {
						uuid=+OS4dgTbyDjODhovSXUAAABV; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIR;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.10 {
						uuid=+OS4dgTbyDjODhovSXUAAABW; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIS;
						x=12000; y=0; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; miry=1; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.11 {
						uuid=+OS4dgTbyDjODhovSXUAAABX; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIT;
						x=12000; y=-16000; rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; rot=180.000000; mirx=1; miry=1; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.12 {
						uuid=+OS4dgTbyDjODhovSXUAAABY; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIU;
						x=0; y=-16000; rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; rot=180.000000; mirx=1; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:text.13 { x1=2000; y1=-2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=T?
					role=symbol
				}
			}
			ha:group.52 {
				uuid=+OS4dgTbyDjODhovSXUAAABZ;
				x=40000; y=464000;
				li:objects {
					ha:line.1 { x1=20000; y1=116000; x2=24000; y2=116000; stroke=wire; }
					ha:line.2 { x1=24000; y1=116000; x2=24000; y2=120000; stroke=wire; }
					ha:line.3 { x1=24000; y1=120000; x2=28000; y2=120000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.55 {
				uuid=+OS4dgTbyDjODhovSXUAAABa;
				x=40000; y=464000;
				li:objects {
					ha:line.1 { x1=20000; y1=112000; x2=24000; y2=112000; stroke=wire; }
					ha:line.2 { x1=24000; y1=112000; x2=24000; y2=108000; stroke=wire; }
					ha:line.3 { x1=24000; y1=108000; x2=28000; y2=108000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.58 {
				uuid=+OS4dgTbyDjODhovSXUAAABg; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIk;
				x=84000; y=532000; rot=270.000000;
				li:objects {
					ha:arc.1 { cx=2000; cy=-12000; r=2000; sang=0.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.2 { cx=6000; cy=-12000; r=2000; sang=0.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.3 { cx=10000; cy=-12000; r=2000; sang=0.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.4 { cx=2000; cy=-4000; r=2000; sang=180.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.5 { cx=6000; cy=-4000; r=2000; sang=180.000000; dang=180.000000; stroke=sym-decor; }
					ha:arc.6 { cx=10000; cy=-4000; r=2000; sang=180.000000; dang=180.000000; stroke=sym-decor; }
					ha:line.7 { x1=0; y1=-9000; x2=12000; y2=-9000; stroke=sym-decor; }
					ha:line.8 { x1=0; y1=-7000; x2=12000; y2=-7000; stroke=sym-decor; }
					ha:group.9 {
						uuid=+OS4dgTbyDjODhovSXUAAABh; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIR;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.10 {
						uuid=+OS4dgTbyDjODhovSXUAAABi; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIS;
						x=12000; y=0; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; miry=1; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.11 {
						uuid=+OS4dgTbyDjODhovSXUAAABj; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIT;
						x=12000; y=-16000; rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; rot=180.000000; mirx=1; miry=1; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.12 {
						uuid=+OS4dgTbyDjODhovSXUAAABk; src_uuid=FJTdh8CXI0wmXfEvXIIAAAIU;
						x=0; y=-16000; rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; rot=180.000000; mirx=1; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:text.13 { x1=2000; y1=-2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=T?
					role=symbol
				}
			}
			ha:group.59 {
				uuid=+OS4dgTbyDjODhovSXUAAABl;
				x=40000; y=432000;
				li:objects {
					ha:line.1 { x1=20000; y1=96000; x2=24000; y2=96000; stroke=wire; }
					ha:line.2 { x1=24000; y1=96000; x2=24000; y2=100000; stroke=wire; }
					ha:line.3 { x1=24000; y1=100000; x2=28000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.62 {
				uuid=+OS4dgTbyDjODhovSXUAAABm;
				x=40000; y=432000;
				li:objects {
					ha:line.1 { x1=20000; y1=92000; x2=24000; y2=92000; stroke=wire; }
					ha:line.2 { x1=24000; y1=92000; x2=24000; y2=88000; stroke=wire; }
					ha:line.3 { x1=24000; y1=88000; x2=28000; y2=88000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.65 {
				uuid=+OS4dgTbyDjODhovSXUAAABq; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=88000; y=532000;
				li:objects {
					ha:group.1 {
						uuid=+OS4dgTbyDjODhovSXUAAABr; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=+OS4dgTbyDjODhovSXUAAABs; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.66 {
				uuid=+OS4dgTbyDjODhovSXUAAABt;
				x=40000; y=464000;
				li:objects {
					ha:line.1 { x1=44000; y1=120000; x2=48000; y2=120000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.69 {
				uuid=+OS4dgTbyDjODhovSXUAAABu;
				x=40000; y=432000;
				li:objects {
					ha:line.1 { x1=44000; y1=100000; x2=48000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.72 {
				uuid=+OS4dgTbyDjODhovSXUAAAB7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=88000; y=568000;
				li:objects {
					ha:group.1 {
						uuid=+OS4dgTbyDjODhovSXUAAAB8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.73 {
				uuid=+OS4dgTbyDjODhovSXUAAAB9; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=88000; y=516000;
				li:objects {
					ha:group.1 {
						uuid=+OS4dgTbyDjODhovSXUAAAB+; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.88 {
				uuid=+OS4dgTbyDjODhovSXUAAAB/;
				x=40000; y=464000;
				li:objects {
					ha:line.1 { x1=44000; y1=108000; x2=48000; y2=108000; stroke=wire; }
					ha:line.2 { x1=48000; y1=108000; x2=48000; y2=104000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.91 {
				uuid=+OS4dgTbyDjODhovSXUAAACA;
				x=40000; y=448000;
				li:objects {
					ha:line.1 { x1=48000; y1=72000; x2=48000; y2=68000; stroke=wire; }
					ha:line.2 { x1=44000; y1=72000; x2=48000; y2=72000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.94 {
				uuid=+OS4dgTbyDjODhovSXUAAACB;
				x=40000; y=464000;
				li:objects {
					ha:line.1 { x1=68000; y1=120000; x2=72000; y2=120000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.97 {
				uuid=+OS4dgTbyDjODhovSXUAAACF; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=112000; y=532000;
				li:objects {
					ha:group.1 {
						uuid=+OS4dgTbyDjODhovSXUAAACG; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=+OS4dgTbyDjODhovSXUAAACH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.98 {
				uuid=+OS4dgTbyDjODhovSXUAAACI;
				x=40000; y=448000;
				li:objects {
					ha:line.1 { x1=68000; y1=84000; x2=72000; y2=84000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.104 {
				uuid=+OS4dgTbyDjODhovSXUAAACN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=140000; y=512000;
				li:objects {
					ha:group.1 {
						uuid=+OS4dgTbyDjODhovSXUAAACO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=+OS4dgTbyDjODhovSXUAAACP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.108 {
				uuid=+OS4dgTbyDjODhovSXUAAACQ;
				x=40000; y=448000;
				li:objects {
					ha:line.1 { x1=100000; y1=64000; x2=96000; y2=64000; stroke=wire; }
					ha:line.2 { x1=92000; y1=84000; x2=100000; y2=84000; stroke=wire; }
					ha:line.3 { x1=96000; y1=64000; x2=96000; y2=84000; stroke=wire; }
					ha:line.4 { x1=96000; y1=84000; x2=96000; y2=84000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.110 {
				uuid=+OS4dgTbyDjODhovSXUAAACR;
				x=40000; y=448000;
				li:objects {
					ha:text.7 { x1=138000; y1=88000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=IN_R_BUF
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.126 {
				uuid=+OS4dgTbyDjODhovSXUAAACW; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=144000; y=564000;
				li:objects {
					ha:group.1 {
						uuid=+OS4dgTbyDjODhovSXUAAACX; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=+OS4dgTbyDjODhovSXUAAACY; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.127 {
				uuid=+OS4dgTbyDjODhovSXUAAACZ;
				x=40000; y=440000;
				li:objects {
					ha:line.1 { x1=104000; y1=124000; x2=96000; y2=124000; stroke=wire; }
					ha:line.2 { x1=92000; y1=144000; x2=100000; y2=144000; stroke=wire; }
					ha:line.3 { x1=96000; y1=124000; x2=96000; y2=144000; stroke=wire; }
					ha:line.4 { x1=96000; y1=144000; x2=96000; y2=144000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.129 {
				uuid=+OS4dgTbyDjODhovSXUAAACa;
				x=40000; y=440000;
				li:objects {
					ha:text.6 { x1=138000; y1=148000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=IN_L_BUF
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.132 {
				uuid=+OS4dgTbyDjODhovSXUAAACb;
				x=40000; y=448000;
				li:objects {
					ha:line.1 { x1=100000; y1=92000; x2=92000; y2=92000; stroke=wire; }
					ha:text.2 { x1=92000; y1=92000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.134 {
				uuid=+OS4dgTbyDjODhovSXUAAACd; src_uuid=+OS4dgTbyDjODhovSXUAAACc;
				x=132000; y=592000;
				li:objects {
					ha:line.1 { x1=8000; y1=0; x2=0; y2=0; stroke=wire; }
					ha:text.2 { x1=0; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.137 {
				uuid=fMIO8VR/GToe4JUvNI8AAABq;
				x=72000; y=80000;
				li:objects {
					ha:line.1 { x1=180000; y1=156000; x2=172000; y2=156000; stroke=wire; }
					ha:text.2 { x1=172000; y1=156000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.139 {
				uuid=fMIO8VR/GToe4JUvNI8AAABr;
				x=80000; y=-4000;
				li:objects {
					ha:line.1 { x1=172000; y1=92000; x2=164000; y2=92000; stroke=wire; }
					ha:text.2 { x1=164000; y1=92000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.143 {
				uuid=fMIO8VR/GToe4JUvNI8AAABv; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=224000; y=80000;
				li:objects {
					ha:group.1 {
						uuid=fMIO8VR/GToe4JUvNI8AAABw; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=fMIO8VR/GToe4JUvNI8AAABx; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.149 {
				uuid=fMIO8VR/GToe4JUvNI8AAAB2; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=224000; y=228000;
				li:objects {
					ha:group.1 {
						uuid=fMIO8VR/GToe4JUvNI8AAAB3; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=fMIO8VR/GToe4JUvNI8AAAB4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.153 {
				uuid=fMIO8VR/GToe4JUvNI8AAAB9; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=252000; y=208000;
				li:objects {
					ha:group.1 {
						uuid=fMIO8VR/GToe4JUvNI8AAAB+; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=fMIO8VR/GToe4JUvNI8AAAB/; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.154 {
				uuid=fMIO8VR/GToe4JUvNI8AAACA; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=252000; y=60000;
				li:objects {
					ha:group.1 {
						uuid=fMIO8VR/GToe4JUvNI8AAACB; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=fMIO8VR/GToe4JUvNI8AAACC; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.157 {
				uuid=fMIO8VR/GToe4JUvNI8AAACE;
				x=72000; y=80000;
				li:objects {
					ha:line.2 { x1=208000; y1=152000; x2=208000; y2=128000; stroke=wire; }
					ha:line.3 { x1=208000; y1=128000; x2=200000; y2=128000; stroke=wire; }
					ha:line.4 { x1=204000; y1=152000; x2=212000; y2=152000; stroke=wire; }
					ha:line.5 { x1=208000; y1=152000; x2=208000; y2=152000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.161 {
				uuid=fMIO8VR/GToe4JUvNI8AAACF;
				x=72000; y=-8000;
				li:objects {
					ha:line.1 { x1=180000; y1=68000; x2=176000; y2=68000; stroke=wire; }
					ha:line.2 { x1=172000; y1=88000; x2=180000; y2=88000; stroke=wire; }
					ha:line.3 { x1=176000; y1=68000; x2=176000; y2=88000; stroke=wire; }
					ha:line.4 { x1=176000; y1=88000; x2=176000; y2=88000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.163 {
				uuid=fMIO8VR/GToe4JUvNI8AAACG;
				x=72000; y=-8000;
				li:objects {
					ha:line.2 { x1=208000; y1=92000; x2=208000; y2=68000; stroke=wire; }
					ha:line.3 { x1=208000; y1=68000; x2=200000; y2=68000; stroke=wire; }
					ha:line.4 { x1=204000; y1=92000; x2=212000; y2=92000; stroke=wire; }
					ha:line.5 { x1=208000; y1=92000; x2=208000; y2=92000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.189 {
				uuid=fMIO8VR/GToe4JUvNI8AAACH;
				x=44000; y=80000;
				li:objects {
					ha:line.1 { x1=208000; y1=128000; x2=204000; y2=128000; stroke=wire; }
					ha:line.2 { x1=204000; y1=128000; x2=204000; y2=148000; stroke=wire; }
					ha:line.3 { x1=200000; y1=148000; x2=208000; y2=148000; stroke=wire; }
					ha:line.4 { x1=204000; y1=148000; x2=204000; y2=148000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.193 {
				uuid=fMIO8VR/GToe4JUvNI8AAADC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB8;
				x=172000; y=144000;
				li:objects {
					ha:group.1 {
						uuid=fMIO8VR/GToe4JUvNI8AAADD; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB9;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vdd; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vdd}
					}
					role=symbol
				}
			}
			ha:group.196 {
				uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=40000; y=440000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.199 {
				uuid=fMIO8VR/GToe4JUvNI8AAADN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=152000; y=576000;
				li:objects {
					ha:group.1 {
						uuid=fMIO8VR/GToe4JUvNI8AAADO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.200 {
				uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=40000; y=440000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.203 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=152000; y=548000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.204 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACM; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=40000; y=388000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.207 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=152000; y=524000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.208 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACP; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=40000; y=388000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.211 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=264000; y=96000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACR; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.212 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACS; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=152000; y=-64000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.215 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACT; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=264000; y=72000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACU; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.216 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACV; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=152000; y=-64000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.219 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACW; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=264000; y=244000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACX; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.220 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACY; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=152000; y=84000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.223 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=264000; y=220000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.224 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACb; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=152000; y=84000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.227 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACf; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=284000; y=84000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACg; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACh; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.228 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAACi; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=284000; y=232000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACj; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAACk; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.237 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAC9; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=208000; y=424000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAC+; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAC/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADB; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=3
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U2
					role=symbol
				}
			}
			ha:group.238 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADD; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=208000; y=364000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADE; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADF; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADI; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=4
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U2
					role=symbol
				}
			}
			ha:group.239 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=128000; y=424000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADM; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADN; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADO; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=1
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U2
					role=symbol
				}
			}
			ha:group.240 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADP; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=128000; y=364000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADR; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADS; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADT; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADU; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=2
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U2
					role=symbol
				}
			}
			ha:group.241 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADb; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=116000; y=376000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.242 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADd; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=4000; y=216000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.243 {
				li:conn {
					/2/242/1
					/2/240/11/1
				}
			}
			ha:connection.244 {
				li:conn {
					/2/242/1
					/2/241/1/1
				}
			}
			ha:group.245 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADe; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=116000; y=352000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADf; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.246 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADg; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=4000; y=216000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.247 {
				li:conn {
					/2/246/1
					/2/240/10/1
				}
			}
			ha:connection.248 {
				li:conn {
					/2/246/1
					/2/245/1/1
				}
			}
			ha:group.249 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADh; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=116000; y=436000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADi; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.250 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADj; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=4000; y=276000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.251 {
				li:conn {
					/2/250/1
					/2/239/11/1
				}
			}
			ha:connection.252 {
				li:conn {
					/2/250/1
					/2/249/1/1
				}
			}
			ha:group.253 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADk; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=116000; y=412000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADl; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.254 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADm; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=4000; y=276000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.255 {
				li:conn {
					/2/254/1
					/2/239/10/1
				}
			}
			ha:connection.256 {
				li:conn {
					/2/254/1
					/2/253/1/1
				}
			}
			ha:group.257 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADn; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=196000; y=436000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADo; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.258 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADp; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=84000; y=276000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.261 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=196000; y=412000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADr; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.262 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADs; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=84000; y=276000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.265 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADt; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=196000; y=376000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADu; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.266 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADv; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=84000; y=216000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.269 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADw; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=196000; y=352000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAADx; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.270 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADy; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=84000; y=216000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.273 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAADz;
				li:objects {
					ha:text.2 { x1=152000; y1=424000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.281 {
				li:conn {
					/2/258/1
					/2/237/11/1
				}
			}
			ha:connection.282 {
				li:conn {
					/2/258/1
					/2/257/1/1
				}
			}
			ha:connection.283 {
				li:conn {
					/2/262/1
					/2/237/10/1
				}
			}
			ha:connection.284 {
				li:conn {
					/2/262/1
					/2/261/1/1
				}
			}
			ha:connection.285 {
				li:conn {
					/2/266/1
					/2/238/11/1
				}
			}
			ha:connection.286 {
				li:conn {
					/2/266/1
					/2/265/1/1
				}
			}
			ha:connection.287 {
				li:conn {
					/2/270/1
					/2/238/10/1
				}
			}
			ha:connection.288 {
				li:conn {
					/2/270/1
					/2/269/1/1
				}
			}
			ha:group.291 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAD6; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=132000; y=424000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAD7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAD8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.295 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAD9;
				li:objects {
					ha:line.1 { x1=132000; y1=424000; x2=128000; y2=424000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.296 {
				li:conn {
					/2/295/1
					/2/239/3/1
				}
			}
			ha:connection.297 {
				li:conn {
					/2/295/1
					/2/291/2/1
				}
			}
			ha:group.300 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAD/;
				li:objects {
					ha:line.1 { x1=100000; y1=404000; x2=156000; y2=404000; stroke=wire; }
					ha:line.2 { x1=156000; y1=424000; x2=156000; y2=424000; stroke=junction; }
					ha:line.3 { x1=156000; y1=404000; x2=156000; y2=424000; stroke=wire; }
					ha:line.4 { x1=100000; y1=420000; x2=100000; y2=404000; stroke=wire; }
					ha:line.5 { x1=152000; y1=424000; x2=160000; y2=424000; stroke=wire; }
					ha:line.6 { x1=104000; y1=420000; x2=100000; y2=420000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.301 {
				li:conn {
					/2/300/5
					/2/291/1/1
				}
			}
			ha:connection.302 {
				li:conn {
					/2/300/6
					/2/239/2/1
				}
			}
			ha:group.303 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEF; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAED;
				x=128000; y=364000;
				li:objects {
					ha:line.1 { x1=4000; y1=0; x2=0; y2=0; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.304 {
				li:conn {
					/2/303/1
					/2/240/3/1
				}
			}
			ha:group.305 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEG; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAEE;
				x=128000; y=364000;
				li:objects {
					ha:line.1 { x1=-28000; y1=-20000; x2=28000; y2=-20000; stroke=wire; }
					ha:line.2 { x1=28000; y1=-20000; x2=28000; y2=0; stroke=wire; }
					ha:line.3 { x1=-28000; y1=-4000; x2=-28000; y2=-20000; stroke=wire; }
					ha:line.4 { x1=-24000; y1=-4000; x2=-28000; y2=-4000; stroke=wire; }
					ha:line.5 { x1=24000; y1=0; x2=32000; y2=0; stroke=wire; }
					ha:line.7 { x1=28000; y1=0; x2=28000; y2=0; stroke=junction; }
					ha:text.8 { x1=24000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.306 {
				li:conn {
					/2/305/4
					/2/240/2/1
				}
			}
			ha:group.307 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=132000; y=364000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:connection.308 {
				li:conn {
					/2/307/1/1
					/2/305/5
				}
			}
			ha:connection.309 {
				li:conn {
					/2/307/2/1
					/2/303/1
				}
			}
			ha:group.324 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEX; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=56000; y=432000; rot=90.000000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEY; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.325 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEd; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=56000; y=404000; rot=90.000000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEe; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEf; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:connection.327 {
				li:conn {
					/2/325/1/1
					/2/331/2
				}
			}
			ha:connection.329 {
				li:conn {
					/2/324/2/1
					/2/331/2
				}
			}
			ha:connection.330 {
				li:conn {
					/2/239/1/1
					/2/331/3
				}
			}
			ha:group.331 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEh;
				li:objects {
					ha:line.1 { x1=104000; y1=368000; x2=76000; y2=368000; stroke=wire; }
					ha:line.2 { x1=56000; y1=424000; x2=56000; y2=432000; stroke=wire; }
					ha:line.3 { x1=56000; y1=428000; x2=104000; y2=428000; stroke=wire; }
					ha:line.4 { x1=56000; y1=428000; x2=56000; y2=428000; stroke=junction; }
					ha:line.5 { x1=76000; y1=368000; x2=76000; y2=428000; stroke=wire; }
					ha:line.6 { x1=76000; y1=428000; x2=76000; y2=428000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.332 {
				li:conn {
					/2/331/1
					/2/240/1/1
				}
			}
			ha:group.335 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEk; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=56000; y=456000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEl; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.336 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEm;
				li:objects {
					ha:line.1 { x1=56000; y1=456000; x2=56000; y2=452000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.337 {
				li:conn {
					/2/336/1
					/2/324/1/1
				}
			}
			ha:connection.338 {
				li:conn {
					/2/336/1
					/2/335/1/1
				}
			}
			ha:group.339 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=56000; y=400000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAEq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.340 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAEr;
				li:objects {
					ha:line.1 { x1=56000; y1=400000; x2=56000; y2=404000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.341 {
				li:conn {
					/2/340/1
					/2/325/2/1
				}
			}
			ha:connection.342 {
				li:conn {
					/2/340/1
					/2/339/1/1
				}
			}
			ha:group.346 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAP5; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=308000; y=232000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAP6; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAP7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.347 {
				uuid=Mn6K7gaZfMoPBT2+M4YAAAP8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=308000; y=84000;
				li:objects {
					ha:group.1 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAP9; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Mn6K7gaZfMoPBT2+M4YAAAP+; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.348 {
				uuid=jpOlME14leBh8Hbw4PQAAAES; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQ3;
				x=176000; y=140000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAET; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAP/;
						x=8000; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0B
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAEU; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQA;
						x=8000; y=-12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W0
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jpOlME14leBh8Hbw4PQAAAEV; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQB;
						x=8000; y=-16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0A
							role=terminal
						}
					}
					ha:group.4 {
						uuid=jpOlME14leBh8Hbw4PQAAAEW; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQC;
						x=8000; y=-24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1B
							role=terminal
						}
					}
					ha:group.5 {
						uuid=jpOlME14leBh8Hbw4PQAAAEX; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQD;
						x=8000; y=-28000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=jpOlME14leBh8Hbw4PQAAAEY; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQE;
						x=8000; y=-32000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1A
							role=terminal
						}
					}
					ha:group.7 {
						uuid=jpOlME14leBh8Hbw4PQAAAEZ; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQF;
						x=-16000; y=-8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SI
							role=terminal
						}
					}
					ha:group.8 {
						uuid=jpOlME14leBh8Hbw4PQAAAEa; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQG;
						x=-16000; y=-12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SO
							role=terminal
						}
					}
					ha:group.9 {
						uuid=jpOlME14leBh8Hbw4PQAAAEb; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQH;
						x=-16000; y=-16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SCK
							role=terminal
						}
					}
					ha:group.10 {
						uuid=jpOlME14leBh8Hbw4PQAAAEc; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQI;
						x=-16000; y=-20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={CS#}
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jpOlME14leBh8Hbw4PQAAAEd; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQJ;
						x=-16000; y=-24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={RS#}
							role=terminal
						}
					}
					ha:group.12 {
						uuid=jpOlME14leBh8Hbw4PQAAAEe; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQK;
						x=-16000; y=-28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={SHDN#}
							role=terminal
						}
					}
					ha:group.13 {
						uuid=jpOlME14leBh8Hbw4PQAAAEf; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQL;
						x=-4000; y=0; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VDD
							role=terminal
						}
					}
					ha:group.14 {
						uuid=jpOlME14leBh8Hbw4PQAAAEg; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQM;
						x=-4000; y=-40000; rot=270.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VSS
							role=terminal
						}
					}
					ha:line.15 { x1=4000; y1=-8000; x2=0; y2=-8000; stroke=sym-decor; }
					ha:line.16 { x1=4000; y1=-16000; x2=0; y2=-16000; stroke=sym-decor; }
					ha:line.17 { x1=0; y1=-8000; x2=0; y2=-9000; stroke=sym-decor; }
					ha:line.18 { x1=0; y1=-16000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.19 { x1=0; y1=-9000; x2=1000; y2=-10000; stroke=sym-decor; }
					ha:line.20 { x1=1000; y1=-10000; x2=-1000; y2=-11000; stroke=sym-decor; }
					ha:line.21 { x1=-1000; y1=-11000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.22 { x1=1000; y1=-12000; x2=-1000; y2=-13000; stroke=sym-decor; }
					ha:line.23 { x1=-1000; y1=-13000; x2=1000; y2=-14000; stroke=sym-decor; }
					ha:line.24 { x1=1000; y1=-14000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.25 { x1=4000; y1=-12000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.26 { x1=1000; y1=-12000; x2=3000; y2=-11000; stroke=sym-decor; }
					ha:line.27 { x1=1000; y1=-12000; x2=3000; y2=-13000; stroke=sym-decor; }
					ha:line.28 { x1=1000; y1=-28000; x2=-1000; y2=-29000; stroke=sym-decor; }
					ha:line.29 { x1=-1000; y1=-29000; x2=1000; y2=-30000; stroke=sym-decor; }
					ha:line.30 { x1=1000; y1=-30000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.31 { x1=4000; y1=-28000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:line.32 { x1=1000; y1=-28000; x2=3000; y2=-27000; stroke=sym-decor; }
					ha:line.33 { x1=1000; y1=-28000; x2=3000; y2=-29000; stroke=sym-decor; }
					ha:line.34 { x1=4000; y1=-24000; x2=0; y2=-24000; stroke=sym-decor; }
					ha:line.35 { x1=4000; y1=-32000; x2=0; y2=-32000; stroke=sym-decor; }
					ha:line.36 { x1=0; y1=-24000; x2=0; y2=-25000; stroke=sym-decor; }
					ha:line.37 { x1=0; y1=-32000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.38 { x1=0; y1=-25000; x2=1000; y2=-26000; stroke=sym-decor; }
					ha:text.39 { x1=-10000; y1=-8000; dyntext=0; stroke=sym-decor; text=SI; }
					ha:text.40 { x1=-10000; y1=-12000; dyntext=0; stroke=sym-decor; text=SO; }
					ha:text.41 { x1=-10000; y1=-16000; dyntext=0; stroke=sym-decor; text=SCK; }
					ha:line.42 { x1=1000; y1=-26000; x2=-1000; y2=-27000; stroke=sym-decor; }
					ha:line.43 { x1=-1000; y1=-27000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:polygon.44 {
						li:outline {
							ha:line { x1=-12000; y1=-4000; x2=-12000; y2=-36000; }
							ha:line { x1=-12000; y1=-36000; x2=4000; y2=-36000; }
							ha:line { x1=4000; y1=-36000; x2=4000; y2=-4000; }
							ha:line { x1=4000; y1=-4000; x2=-12000; y2=-4000; }
						}
						stroke=sym-decor;
					}
					ha:text.45 { x1=0; y1=-8000; dyntext=0; stroke=sym-decor; text=0B; }
					ha:text.46 { x1=0; y1=-19000; dyntext=0; stroke=sym-decor; text=0A; }
					ha:text.47 { x1=0; y1=-24000; dyntext=0; stroke=sym-decor; text=1B; }
					ha:text.48 { x1=-10000; y1=-20000; dyntext=0; stroke=sym-decor; text=CS; }
					ha:text.49 { x1=-10000; y1=-24000; dyntext=0; stroke=sym-decor; text=RS; }
					ha:text.50 { x1=-10000; y1=-28000; dyntext=0; stroke=sym-decor; text=SHDN; }
					ha:text.51 { x1=0; y1=-35000; dyntext=0; stroke=sym-decor; text=1A; }
					ha:text.52 { x1=-2000; y1=-9000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VDD; }
					ha:text.53 { x1=-2000; y1=-35000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VSS; }
					ha:text.54 { x1=-2000; y1=-3000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=U3
					role=symbol
					value=10k
				}
			}
			ha:group.349 {
				uuid=jpOlME14leBh8Hbw4PQAAAEl; src_uuid=iNOQfJpO6hT/HFDFGjoAAACA;
				x=172000; y=96000; rot=180.000000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAEm; src_uuid=iNOQfJpO6hT/HFDFGjoAAACB;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vss; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vss}
					}
					role=symbol
				}
			}
			ha:group.352 {
				uuid=jpOlME14leBh8Hbw4PQAAAEn;
				x=32000; y=-84000;
				li:objects {
					ha:line.1 { x1=140000; y1=184000; x2=140000; y2=180000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.355 {
				uuid=jpOlME14leBh8Hbw4PQAAAEo;
				x=32000; y=-84000;
				li:objects {
					ha:line.1 { x1=140000; y1=228000; x2=140000; y2=224000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.422 {
				li:conn {
					/2/220/1
					/2/4/11/1
				}
			}
			ha:connection.424 {
				li:conn {
					/2/224/1
					/2/223/1/1
				}
			}
			ha:group.440 {
				uuid=jpOlME14leBh8Hbw4PQAAAEr;
				x=-16000; y=36000;
				li:objects {
					ha:line.1 { x1=200000; y1=156000; x2=212000; y2=156000; stroke=wire; }
					ha:text.2 { x1=208000; y1=156000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.449 {
				uuid=jpOlME14leBh8Hbw4PQAAAEw; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=152000; y=600000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAEx; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.457 {
				uuid=jpOlME14leBh8Hbw4PQAAAE1;
				x=0; y=-8000;
				li:objects {
					ha:line.1 { x1=304000; y1=92000; x2=308000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.460 {
				uuid=jpOlME14leBh8Hbw4PQAAAE2;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=304000; y1=224000; x2=308000; y2=224000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.463 {
				uuid=jpOlME14leBh8Hbw4PQAAAFw; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=276000; y=184000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAFx; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAFy; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jpOlME14leBh8Hbw4PQAAAFz; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=jpOlME14leBh8Hbw4PQAAAF0; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jpOlME14leBh8Hbw4PQAAAF1; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=2
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U4
					role=symbol
				}
			}
			ha:group.464 {
				uuid=jpOlME14leBh8Hbw4PQAAAF2; src_uuid=fMIO8VR/GToe4JUvNI8AAABq;
				x=72000; y=32000;
				li:objects {
					ha:line.1 { x1=180000; y1=156000; x2=172000; y2=156000; stroke=wire; }
					ha:text.2 { x1=172000; y1=156000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.466 {
				uuid=jpOlME14leBh8Hbw4PQAAAF3; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=224000; y=180000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAF4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAF5; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.467 {
				uuid=jpOlME14leBh8Hbw4PQAAAF6; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=252000; y=160000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAF7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAF8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=10000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.468 {
				uuid=jpOlME14leBh8Hbw4PQAAAF9; src_uuid=fMIO8VR/GToe4JUvNI8AAACE;
				x=72000; y=32000;
				li:objects {
					ha:line.2 { x1=208000; y1=152000; x2=208000; y2=128000; stroke=wire; }
					ha:line.3 { x1=208000; y1=128000; x2=200000; y2=128000; stroke=wire; }
					ha:line.4 { x1=204000; y1=152000; x2=212000; y2=152000; stroke=wire; }
					ha:line.5 { x1=208000; y1=152000; x2=208000; y2=152000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.471 {
				uuid=jpOlME14leBh8Hbw4PQAAAF+; src_uuid=fMIO8VR/GToe4JUvNI8AAACH;
				x=44000; y=32000;
				li:objects {
					ha:line.1 { x1=208000; y1=128000; x2=204000; y2=128000; stroke=wire; }
					ha:line.2 { x1=204000; y1=128000; x2=204000; y2=148000; stroke=wire; }
					ha:line.3 { x1=200000; y1=148000; x2=208000; y2=148000; stroke=wire; }
					ha:line.4 { x1=204000; y1=148000; x2=204000; y2=148000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.475 {
				uuid=jpOlME14leBh8Hbw4PQAAAF/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=264000; y=196000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAGA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.476 {
				uuid=jpOlME14leBh8Hbw4PQAAAGB; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=152000; y=36000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.477 {
				li:conn {
					/2/476/1
					/2/463/11/1
				}
			}
			ha:group.479 {
				uuid=jpOlME14leBh8Hbw4PQAAAGC; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=264000; y=172000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAGD; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.480 {
				uuid=jpOlME14leBh8Hbw4PQAAAGE; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=152000; y=36000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.482 {
				li:conn {
					/2/480/1
					/2/479/1/1
				}
			}
			ha:group.483 {
				uuid=jpOlME14leBh8Hbw4PQAAAGF; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=284000; y=184000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAGG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAGH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.485 {
				uuid=jpOlME14leBh8Hbw4PQAAAGI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=308000; y=184000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAGJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAGK; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.486 {
				uuid=jpOlME14leBh8Hbw4PQAAAGL; src_uuid=jpOlME14leBh8Hbw4PQAAAE2;
				x=0; y=-40000;
				li:objects {
					ha:line.1 { x1=304000; y1=224000; x2=308000; y2=224000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.489 {
				uuid=jpOlME14leBh8Hbw4PQAAAGo; src_uuid=fMIO8VR/GToe4JUvNI8AAABr;
				x=80000; y=44000;
				li:objects {
					ha:line.1 { x1=172000; y1=92000; x2=164000; y2=92000; stroke=wire; }
					ha:text.2 { x1=164000; y1=92000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.490 {
				uuid=jpOlME14leBh8Hbw4PQAAAGp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=224000; y=128000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAGq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAGr; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.491 {
				uuid=jpOlME14leBh8Hbw4PQAAAGs; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=252000; y=108000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAGt; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAGu; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.492 {
				uuid=jpOlME14leBh8Hbw4PQAAAGv; src_uuid=fMIO8VR/GToe4JUvNI8AAACF;
				x=72000; y=40000;
				li:objects {
					ha:line.1 { x1=180000; y1=68000; x2=176000; y2=68000; stroke=wire; }
					ha:line.2 { x1=172000; y1=88000; x2=180000; y2=88000; stroke=wire; }
					ha:line.3 { x1=176000; y1=68000; x2=176000; y2=88000; stroke=wire; }
					ha:line.4 { x1=176000; y1=88000; x2=176000; y2=88000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.495 {
				uuid=jpOlME14leBh8Hbw4PQAAAGw; src_uuid=fMIO8VR/GToe4JUvNI8AAACG;
				x=72000; y=40000;
				li:objects {
					ha:line.2 { x1=208000; y1=92000; x2=208000; y2=68000; stroke=wire; }
					ha:line.3 { x1=208000; y1=68000; x2=200000; y2=68000; stroke=wire; }
					ha:line.4 { x1=204000; y1=92000; x2=212000; y2=92000; stroke=wire; }
					ha:line.5 { x1=208000; y1=92000; x2=208000; y2=92000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.497 {
				uuid=jpOlME14leBh8Hbw4PQAAAGx; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=264000; y=144000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAGy; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.498 {
				uuid=jpOlME14leBh8Hbw4PQAAAGz; src_uuid=fMIO8VR/GToe4JUvNI8AAADK;
				x=152000; y=-16000;
				li:objects {
					ha:line.1 { x1=112000; y1=160000; x2=112000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.500 {
				uuid=jpOlME14leBh8Hbw4PQAAAG0; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=264000; y=120000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAG1; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.501 {
				uuid=jpOlME14leBh8Hbw4PQAAAG2; src_uuid=fMIO8VR/GToe4JUvNI8AAADP;
				x=152000; y=-16000;
				li:objects {
					ha:line.1 { x1=112000; y1=136000; x2=112000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.503 {
				uuid=jpOlME14leBh8Hbw4PQAAAG3; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=284000; y=132000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAG4; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAG5; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.505 {
				uuid=jpOlME14leBh8Hbw4PQAAAG6; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=308000; y=132000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAG7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAG8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=<n/a>
				}
			}
			ha:group.506 {
				uuid=jpOlME14leBh8Hbw4PQAAAG9; src_uuid=jpOlME14leBh8Hbw4PQAAAE1;
				x=0; y=40000;
				li:objects {
					ha:line.1 { x1=304000; y1=92000; x2=308000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.509 {
				uuid=jpOlME14leBh8Hbw4PQAAAG+; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=276000; y=132000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAG/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAHA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jpOlME14leBh8Hbw4PQAAAHB; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=jpOlME14leBh8Hbw4PQAAAHC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jpOlME14leBh8Hbw4PQAAAHD; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=3
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U4
					role=symbol
				}
			}
			ha:connection.531 {
				li:conn {
					/2/137/1
					/2/4/1/1
				}
			}
			ha:connection.532 {
				li:conn {
					/2/157/3
					/2/153/1/1
				}
			}
			ha:connection.533 {
				li:conn {
					/2/157/4
					/2/4/3/1
				}
			}
			ha:connection.534 {
				li:conn {
					/2/189/1
					/2/153/2/1
				}
			}
			ha:connection.535 {
				li:conn {
					/2/189/3
					/2/149/1/1
				}
			}
			ha:connection.536 {
				li:conn {
					/2/189/3
					/2/4/2/1
				}
			}
			ha:connection.537 {
				li:conn {
					/2/220/1
					/2/219/1/1
				}
			}
			ha:connection.538 {
				li:conn {
					/2/224/1
					/2/4/10/1
				}
			}
			ha:connection.539 {
				li:conn {
					/2/228/2/1
					/2/157/4
				}
			}
			ha:connection.540 {
				li:conn {
					/2/460/1
					/2/346/2/1
				}
			}
			ha:connection.541 {
				li:conn {
					/2/460/1
					/2/228/1/1
				}
			}
			ha:connection.544 {
				li:conn {
					/2/464/1
					/2/463/1/1
				}
			}
			ha:connection.545 {
				li:conn {
					/2/468/4
					/2/463/3/1
				}
			}
			ha:connection.546 {
				li:conn {
					/2/471/3
					/2/463/2/1
				}
			}
			ha:connection.547 {
				li:conn {
					/2/471/3
					/2/466/1/1
				}
			}
			ha:connection.548 {
				li:conn {
					/2/476/1
					/2/475/1/1
				}
			}
			ha:connection.549 {
				li:conn {
					/2/480/1
					/2/463/10/1
				}
			}
			ha:connection.550 {
				li:conn {
					/2/483/2/1
					/2/468/4
				}
			}
			ha:connection.551 {
				li:conn {
					/2/486/1
					/2/483/1/1
				}
			}
			ha:connection.552 {
				li:conn {
					/2/486/1
					/2/485/2/1
				}
			}
			ha:connection.555 {
				li:conn {
					/2/149/2/1
					/2/653/2
				}
			}
			ha:connection.572 {
				li:conn {
					/2/467/1/1
					/2/468/3
				}
			}
			ha:connection.573 {
				li:conn {
					/2/467/2/1
					/2/471/1
				}
			}
			ha:group.576 {
				uuid=jpOlME14leBh8Hbw4PQAAAHb; src_uuid=jpOlME14leBh8Hbw4PQAAAEo;
				x=16000; y=-12000;
				li:objects {
					ha:line.1 { x1=156000; y1=224000; x2=156000; y2=220000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.577 {
				uuid=jpOlME14leBh8Hbw4PQAAAHc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB8;
				x=172000; y=212000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAHd; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB9;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vdd; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vdd}
					}
					role=symbol
				}
			}
			ha:group.580 {
				uuid=jpOlME14leBh8Hbw4PQAAAHe; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQ3;
				x=176000; y=208000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAHf; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAP/;
						x=8000; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0B
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jpOlME14leBh8Hbw4PQAAAHg; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQA;
						x=8000; y=-12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W0
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jpOlME14leBh8Hbw4PQAAAHh; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQB;
						x=8000; y=-16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0A
							role=terminal
						}
					}
					ha:group.4 {
						uuid=jpOlME14leBh8Hbw4PQAAAHi; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQC;
						x=8000; y=-24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1B
							role=terminal
						}
					}
					ha:group.5 {
						uuid=jpOlME14leBh8Hbw4PQAAAHj; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQD;
						x=8000; y=-28000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=jpOlME14leBh8Hbw4PQAAAHk; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQE;
						x=8000; y=-32000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1A
							role=terminal
						}
					}
					ha:group.7 {
						uuid=jpOlME14leBh8Hbw4PQAAAHl; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQF;
						x=-16000; y=-8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SI
							role=terminal
						}
					}
					ha:group.8 {
						uuid=jpOlME14leBh8Hbw4PQAAAHm; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQG;
						x=-16000; y=-12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SO
							role=terminal
						}
					}
					ha:group.9 {
						uuid=jpOlME14leBh8Hbw4PQAAAHn; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQH;
						x=-16000; y=-16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SCK
							role=terminal
						}
					}
					ha:group.10 {
						uuid=jpOlME14leBh8Hbw4PQAAAHo; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQI;
						x=-16000; y=-20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={CS#}
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jpOlME14leBh8Hbw4PQAAAHp; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQJ;
						x=-16000; y=-24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={RS#}
							role=terminal
						}
					}
					ha:group.12 {
						uuid=jpOlME14leBh8Hbw4PQAAAHq; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQK;
						x=-16000; y=-28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={SHDN#}
							role=terminal
						}
					}
					ha:group.13 {
						uuid=jpOlME14leBh8Hbw4PQAAAHr; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQL;
						x=-4000; y=0; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VDD
							role=terminal
						}
					}
					ha:group.14 {
						uuid=jpOlME14leBh8Hbw4PQAAAHs; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQM;
						x=-4000; y=-40000; rot=270.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VSS
							role=terminal
						}
					}
					ha:line.15 { x1=4000; y1=-8000; x2=0; y2=-8000; stroke=sym-decor; }
					ha:line.16 { x1=4000; y1=-16000; x2=0; y2=-16000; stroke=sym-decor; }
					ha:line.17 { x1=0; y1=-8000; x2=0; y2=-9000; stroke=sym-decor; }
					ha:line.18 { x1=0; y1=-16000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.19 { x1=0; y1=-9000; x2=1000; y2=-10000; stroke=sym-decor; }
					ha:line.20 { x1=1000; y1=-10000; x2=-1000; y2=-11000; stroke=sym-decor; }
					ha:line.21 { x1=-1000; y1=-11000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.22 { x1=1000; y1=-12000; x2=-1000; y2=-13000; stroke=sym-decor; }
					ha:line.23 { x1=-1000; y1=-13000; x2=1000; y2=-14000; stroke=sym-decor; }
					ha:line.24 { x1=1000; y1=-14000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.25 { x1=4000; y1=-12000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.26 { x1=1000; y1=-12000; x2=3000; y2=-11000; stroke=sym-decor; }
					ha:line.27 { x1=1000; y1=-12000; x2=3000; y2=-13000; stroke=sym-decor; }
					ha:line.28 { x1=1000; y1=-28000; x2=-1000; y2=-29000; stroke=sym-decor; }
					ha:line.29 { x1=-1000; y1=-29000; x2=1000; y2=-30000; stroke=sym-decor; }
					ha:line.30 { x1=1000; y1=-30000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.31 { x1=4000; y1=-28000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:line.32 { x1=1000; y1=-28000; x2=3000; y2=-27000; stroke=sym-decor; }
					ha:line.33 { x1=1000; y1=-28000; x2=3000; y2=-29000; stroke=sym-decor; }
					ha:line.34 { x1=4000; y1=-24000; x2=0; y2=-24000; stroke=sym-decor; }
					ha:line.35 { x1=4000; y1=-32000; x2=0; y2=-32000; stroke=sym-decor; }
					ha:line.36 { x1=0; y1=-24000; x2=0; y2=-25000; stroke=sym-decor; }
					ha:line.37 { x1=0; y1=-32000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.38 { x1=0; y1=-25000; x2=1000; y2=-26000; stroke=sym-decor; }
					ha:text.39 { x1=-10000; y1=-8000; dyntext=0; stroke=sym-decor; text=SI; }
					ha:text.40 { x1=-10000; y1=-12000; dyntext=0; stroke=sym-decor; text=SO; }
					ha:text.41 { x1=-10000; y1=-16000; dyntext=0; stroke=sym-decor; text=SCK; }
					ha:line.42 { x1=1000; y1=-26000; x2=-1000; y2=-27000; stroke=sym-decor; }
					ha:line.43 { x1=-1000; y1=-27000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:polygon.44 {
						li:outline {
							ha:line { x1=-12000; y1=-4000; x2=-12000; y2=-36000; }
							ha:line { x1=-12000; y1=-36000; x2=4000; y2=-36000; }
							ha:line { x1=4000; y1=-36000; x2=4000; y2=-4000; }
							ha:line { x1=4000; y1=-4000; x2=-12000; y2=-4000; }
						}
						stroke=sym-decor;
					}
					ha:text.45 { x1=0; y1=-8000; dyntext=0; stroke=sym-decor; text=0B; }
					ha:text.46 { x1=0; y1=-19000; dyntext=0; stroke=sym-decor; text=0A; }
					ha:text.47 { x1=0; y1=-24000; dyntext=0; stroke=sym-decor; text=1B; }
					ha:text.48 { x1=-10000; y1=-20000; dyntext=0; stroke=sym-decor; text=CS; }
					ha:text.49 { x1=-10000; y1=-24000; dyntext=0; stroke=sym-decor; text=RS; }
					ha:text.50 { x1=-10000; y1=-28000; dyntext=0; stroke=sym-decor; text=SHDN; }
					ha:text.51 { x1=0; y1=-35000; dyntext=0; stroke=sym-decor; text=1A; }
					ha:text.52 { x1=-2000; y1=-9000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VDD; }
					ha:text.53 { x1=-2000; y1=-35000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VSS; }
					ha:text.54 { x1=-2000; y1=-3000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=U5
					role=symbol
					value=10k
				}
			}
			ha:group.582 {
				uuid=jpOlME14leBh8Hbw4PQAAAHt; src_uuid=iNOQfJpO6hT/HFDFGjoAAACA;
				x=172000; y=164000; rot=180.000000;
				li:objects {
					ha:group.1 {
						uuid=jpOlME14leBh8Hbw4PQAAAHu; src_uuid=iNOQfJpO6hT/HFDFGjoAAACB;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vss; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vss}
					}
					role=symbol
				}
			}
			ha:group.583 {
				uuid=jpOlME14leBh8Hbw4PQAAAHv; src_uuid=jpOlME14leBh8Hbw4PQAAAEn;
				x=32000; y=8000;
				li:objects {
					ha:line.1 { x1=140000; y1=160000; x2=140000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.604 {
				li:conn {
					/2/352/1
					/2/348/14/1
				}
			}
			ha:connection.606 {
				li:conn {
					/2/355/1
					/2/193/1/1
				}
			}
			ha:connection.614 {
				li:conn {
					/2/352/1
					/2/349/1/1
				}
			}
			ha:connection.615 {
				li:conn {
					/2/355/1
					/2/348/13/1
				}
			}
			ha:connection.616 {
				li:conn {
					/2/577/1/1
					/2/576/1
				}
			}
			ha:connection.619 {
				li:conn {
					/2/583/1
					/2/582/1/1
				}
			}
			ha:connection.620 {
				li:conn {
					/2/580/13/1
					/2/576/1
				}
			}
			ha:connection.621 {
				li:conn {
					/2/583/1
					/2/580/14/1
				}
			}
			ha:connection.624 {
				li:conn {
					/2/440/1
					/2/580/3/1
				}
			}
			ha:group.626 {
				uuid=jpOlME14leBh8Hbw4PQAAAHx;
				li:objects {
					ha:line.2 { x1=184000; y1=128000; x2=224000; y2=128000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.631 {
				uuid=jpOlME14leBh8Hbw4PQAAAH0; src_uuid=jpOlME14leBh8Hbw4PQAAAHz;
				x=184000; y=108000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=12000; y2=0; stroke=wire; }
					ha:text.2 { x1=6000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.632 {
				li:conn {
					/2/631/1
					/2/348/6/1
				}
			}
			ha:group.633 {
				uuid=jpOlME14leBh8Hbw4PQAAAH1; src_uuid=jpOlME14leBh8Hbw4PQAAAHz;
				x=184000; y=124000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=12000; y2=0; stroke=wire; }
					ha:text.2 { x1=6000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.634 {
				li:conn {
					/2/633/1
					/2/348/3/1
				}
			}
			ha:group.635 {
				uuid=jpOlME14leBh8Hbw4PQAAAH3; src_uuid=jpOlME14leBh8Hbw4PQAAAH2;
				x=184000; y=176000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=12000; y2=0; stroke=wire; }
					ha:text.2 { x1=8000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.636 {
				li:conn {
					/2/635/1
					/2/580/6/1
				}
			}
			ha:group.644 {
				uuid=jpOlME14leBh8Hbw4PQAAAH6;
				li:objects {
					ha:line.4 { x1=188000; y1=184000; x2=196000; y2=184000; stroke=wire; }
					ha:line.6 { x1=184000; y1=184000; x2=188000; y2=184000; stroke=wire; }
					ha:text.9 { x1=192000; y1=184000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
					ha:line.10 { x1=184000; y1=200000; x2=188000; y2=200000; stroke=wire; }
					ha:line.11 { x1=188000; y1=200000; x2=188000; y2=184000; stroke=wire; }
					ha:line.12 { x1=188000; y1=184000; x2=188000; y2=184000; stroke=junction; }
				}
				ha:attrib {
					name=L_BAX
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.648 {
				li:conn {
					/2/644/6
					/2/580/4/1
				}
			}
			ha:group.651 {
				uuid=jpOlME14leBh8Hbw4PQAAAH7;
				li:objects {
					ha:line.3 { x1=184000; y1=180000; x2=224000; y2=180000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.652 {
				li:conn {
					/2/466/2/1
					/2/651/3
				}
			}
			ha:group.653 {
				uuid=jpOlME14leBh8Hbw4PQAAAH8;
				li:objects {
					ha:line.1 { x1=184000; y1=196000; x2=216000; y2=196000; stroke=wire; }
					ha:line.2 { x1=216000; y1=228000; x2=224000; y2=228000; stroke=wire; }
					ha:line.3 { x1=216000; y1=196000; x2=216000; y2=228000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.654 {
				li:conn {
					/2/653/1
					/2/580/2/1
				}
			}
			ha:connection.655 {
				li:conn {
					/2/651/3
					/2/580/5/1
				}
			}
			ha:connection.663 {
				li:conn {
					/2/212/1
					/2/5/11/1
				}
			}
			ha:connection.664 {
				li:conn {
					/2/216/1
					/2/215/1/1
				}
			}
			ha:connection.672 {
				li:conn {
					/2/501/1
					/2/500/1/1
				}
			}
			ha:connection.679 {
				li:conn {
					/2/509/10/1
					/2/501/1
				}
			}
			ha:connection.683 {
				li:conn {
					/2/139/1
					/2/5/1/1
				}
			}
			ha:connection.684 {
				li:conn {
					/2/161/1
					/2/154/2/1
				}
			}
			ha:connection.685 {
				li:conn {
					/2/161/2
					/2/143/1/1
				}
			}
			ha:connection.686 {
				li:conn {
					/2/161/2
					/2/5/2/1
				}
			}
			ha:connection.687 {
				li:conn {
					/2/163/3
					/2/154/1/1
				}
			}
			ha:connection.688 {
				li:conn {
					/2/163/4
					/2/5/3/1
				}
			}
			ha:connection.689 {
				li:conn {
					/2/212/1
					/2/211/1/1
				}
			}
			ha:connection.690 {
				li:conn {
					/2/216/1
					/2/5/10/1
				}
			}
			ha:connection.691 {
				li:conn {
					/2/227/2/1
					/2/163/4
				}
			}
			ha:connection.692 {
				li:conn {
					/2/457/1
					/2/227/1/1
				}
			}
			ha:connection.693 {
				li:conn {
					/2/457/1
					/2/347/2/1
				}
			}
			ha:connection.694 {
				li:conn {
					/2/492/1
					/2/491/2/1
				}
			}
			ha:connection.695 {
				li:conn {
					/2/492/2
					/2/490/1/1
				}
			}
			ha:connection.696 {
				li:conn {
					/2/495/3
					/2/491/1/1
				}
			}
			ha:connection.697 {
				li:conn {
					/2/498/1
					/2/497/1/1
				}
			}
			ha:connection.699 {
				li:conn {
					/2/503/2/1
					/2/495/4
				}
			}
			ha:connection.700 {
				li:conn {
					/2/506/1
					/2/503/1/1
				}
			}
			ha:connection.701 {
				li:conn {
					/2/506/1
					/2/505/2/1
				}
			}
			ha:connection.702 {
				li:conn {
					/2/509/1/1
					/2/489/1
				}
			}
			ha:connection.703 {
				li:conn {
					/2/509/2/1
					/2/492/2
				}
			}
			ha:connection.704 {
				li:conn {
					/2/509/3/1
					/2/495/4
				}
			}
			ha:connection.705 {
				li:conn {
					/2/509/11/1
					/2/498/1
				}
			}
			ha:connection.706 {
				li:conn {
					/2/143/2/1
					/2/709/2
				}
			}
			ha:connection.707 {
				li:conn {
					/2/490/2/1
					/2/626/2
				}
			}
			ha:connection.708 {
				li:conn {
					/2/626/2
					/2/348/2/1
				}
			}
			ha:group.709 {
				uuid=jpOlME14leBh8Hbw4PQAAAH9;
				li:objects {
					ha:line.1 { x1=184000; y1=112000; x2=216000; y2=112000; stroke=wire; }
					ha:line.2 { x1=216000; y1=80000; x2=224000; y2=80000; stroke=wire; }
					ha:line.3 { x1=216000; y1=112000; x2=216000; y2=80000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.710 {
				li:conn {
					/2/709/1
					/2/348/5/1
				}
			}
			ha:group.720 {
				uuid=jsnwx40oi7YLGck+mCkAAAFl; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=452000; y=452000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAFm; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAFn; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jsnwx40oi7YLGck+mCkAAAFo; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=jsnwx40oi7YLGck+mCkAAAFp; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jsnwx40oi7YLGck+mCkAAAFq; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=3
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U?
					role=symbol
				}
			}
			ha:group.721 {
				uuid=jsnwx40oi7YLGck+mCkAAAFu; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=292000; y=488000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAFv; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAFw; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=10k
				}
			}
			ha:group.722 {
				uuid=jsnwx40oi7YLGck+mCkAAAF0; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=264000; y=452000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAF1; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAF2; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=10uF
				}
			}
			ha:group.723 {
				uuid=jsnwx40oi7YLGck+mCkAAAF6; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=292000; y=416000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAF7; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAF8; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:group.724 {
				uuid=jsnwx40oi7YLGck+mCkAAAGM; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQ3;
				x=324000; y=468000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAGN; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAP/;
						x=8000; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0B
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAGO; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQA;
						x=8000; y=-12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W0
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jsnwx40oi7YLGck+mCkAAAGP; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQB;
						x=8000; y=-16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0A
							role=terminal
						}
					}
					ha:group.4 {
						uuid=jsnwx40oi7YLGck+mCkAAAGQ; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQC;
						x=8000; y=-24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1B
							role=terminal
						}
					}
					ha:group.5 {
						uuid=jsnwx40oi7YLGck+mCkAAAGR; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQD;
						x=8000; y=-28000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=jsnwx40oi7YLGck+mCkAAAGS; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQE;
						x=8000; y=-32000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1A
							role=terminal
						}
					}
					ha:group.7 {
						uuid=jsnwx40oi7YLGck+mCkAAAGT; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQF;
						x=-16000; y=-8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SI
							role=terminal
						}
					}
					ha:group.8 {
						uuid=jsnwx40oi7YLGck+mCkAAAGU; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQG;
						x=-16000; y=-12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SO
							role=terminal
						}
					}
					ha:group.9 {
						uuid=jsnwx40oi7YLGck+mCkAAAGV; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQH;
						x=-16000; y=-16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SCK
							role=terminal
						}
					}
					ha:group.10 {
						uuid=jsnwx40oi7YLGck+mCkAAAGW; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQI;
						x=-16000; y=-20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={CS#}
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jsnwx40oi7YLGck+mCkAAAGX; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQJ;
						x=-16000; y=-24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={RS#}
							role=terminal
						}
					}
					ha:group.12 {
						uuid=jsnwx40oi7YLGck+mCkAAAGY; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQK;
						x=-16000; y=-28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={SHDN#}
							role=terminal
						}
					}
					ha:group.13 {
						uuid=jsnwx40oi7YLGck+mCkAAAGZ; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQL;
						x=-4000; y=0; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VDD
							role=terminal
						}
					}
					ha:group.14 {
						uuid=jsnwx40oi7YLGck+mCkAAAGa; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQM;
						x=-4000; y=-40000; rot=270.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VSS
							role=terminal
						}
					}
					ha:line.15 { x1=4000; y1=-8000; x2=0; y2=-8000; stroke=sym-decor; }
					ha:line.16 { x1=4000; y1=-16000; x2=0; y2=-16000; stroke=sym-decor; }
					ha:line.17 { x1=0; y1=-8000; x2=0; y2=-9000; stroke=sym-decor; }
					ha:line.18 { x1=0; y1=-16000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.19 { x1=0; y1=-9000; x2=1000; y2=-10000; stroke=sym-decor; }
					ha:line.20 { x1=1000; y1=-10000; x2=-1000; y2=-11000; stroke=sym-decor; }
					ha:line.21 { x1=-1000; y1=-11000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.22 { x1=1000; y1=-12000; x2=-1000; y2=-13000; stroke=sym-decor; }
					ha:line.23 { x1=-1000; y1=-13000; x2=1000; y2=-14000; stroke=sym-decor; }
					ha:line.24 { x1=1000; y1=-14000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.25 { x1=4000; y1=-12000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.26 { x1=1000; y1=-12000; x2=3000; y2=-11000; stroke=sym-decor; }
					ha:line.27 { x1=1000; y1=-12000; x2=3000; y2=-13000; stroke=sym-decor; }
					ha:line.28 { x1=1000; y1=-28000; x2=-1000; y2=-29000; stroke=sym-decor; }
					ha:line.29 { x1=-1000; y1=-29000; x2=1000; y2=-30000; stroke=sym-decor; }
					ha:line.30 { x1=1000; y1=-30000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.31 { x1=4000; y1=-28000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:line.32 { x1=1000; y1=-28000; x2=3000; y2=-27000; stroke=sym-decor; }
					ha:line.33 { x1=1000; y1=-28000; x2=3000; y2=-29000; stroke=sym-decor; }
					ha:line.34 { x1=4000; y1=-24000; x2=0; y2=-24000; stroke=sym-decor; }
					ha:line.35 { x1=4000; y1=-32000; x2=0; y2=-32000; stroke=sym-decor; }
					ha:line.36 { x1=0; y1=-24000; x2=0; y2=-25000; stroke=sym-decor; }
					ha:line.37 { x1=0; y1=-32000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.38 { x1=0; y1=-25000; x2=1000; y2=-26000; stroke=sym-decor; }
					ha:text.39 { x1=-10000; y1=-8000; dyntext=0; stroke=sym-decor; text=SI; }
					ha:text.40 { x1=-10000; y1=-12000; dyntext=0; stroke=sym-decor; text=SO; }
					ha:text.41 { x1=-10000; y1=-16000; dyntext=0; stroke=sym-decor; text=SCK; }
					ha:line.42 { x1=1000; y1=-26000; x2=-1000; y2=-27000; stroke=sym-decor; }
					ha:line.43 { x1=-1000; y1=-27000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:polygon.44 {
						li:outline {
							ha:line { x1=-12000; y1=-4000; x2=-12000; y2=-36000; }
							ha:line { x1=-12000; y1=-36000; x2=4000; y2=-36000; }
							ha:line { x1=4000; y1=-36000; x2=4000; y2=-4000; }
							ha:line { x1=4000; y1=-4000; x2=-12000; y2=-4000; }
						}
						stroke=sym-decor;
					}
					ha:text.45 { x1=0; y1=-8000; dyntext=0; stroke=sym-decor; text=0B; }
					ha:text.46 { x1=0; y1=-19000; dyntext=0; stroke=sym-decor; text=0A; }
					ha:text.47 { x1=0; y1=-24000; dyntext=0; stroke=sym-decor; text=1B; }
					ha:text.48 { x1=-10000; y1=-20000; dyntext=0; stroke=sym-decor; text=CS; }
					ha:text.49 { x1=-10000; y1=-24000; dyntext=0; stroke=sym-decor; text=RS; }
					ha:text.50 { x1=-10000; y1=-28000; dyntext=0; stroke=sym-decor; text=SHDN; }
					ha:text.51 { x1=0; y1=-35000; dyntext=0; stroke=sym-decor; text=1A; }
					ha:text.52 { x1=-2000; y1=-9000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VDD; }
					ha:text.53 { x1=-2000; y1=-35000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VSS; }
					ha:text.54 { x1=-2000; y1=-3000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=U?
					role=symbol
					value=100k
				}
			}
			ha:group.731 {
				uuid=jsnwx40oi7YLGck+mCkAAAGd;
				li:objects {
					ha:line.1 { x1=332000; y1=452000; x2=352000; y2=452000; stroke=wire; }
					ha:line.7 { x1=352000; y1=452000; x2=352000; y2=488000; stroke=wire; }
					ha:line.11 { x1=360000; y1=484000; x2=360000; y2=488000; stroke=wire; }
					ha:line.12 { x1=352000; y1=488000; x2=364000; y2=488000; stroke=wire; }
					ha:line.15 { x1=360000; y1=488000; x2=360000; y2=488000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.733 {
				uuid=jsnwx40oi7YLGck+mCkAAAGh; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=364000; y=488000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAGi; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAGj; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=10k
				}
			}
			ha:group.737 {
				uuid=jsnwx40oi7YLGck+mCkAAAGl;
				li:objects {
					ha:line.4 { x1=348000; y1=416000; x2=348000; y2=444000; stroke=wire; }
					ha:line.5 { x1=332000; y1=444000; x2=348000; y2=444000; stroke=wire; }
					ha:line.6 { x1=312000; y1=416000; x2=360000; y2=416000; stroke=wire; }
					ha:line.15 { x1=348000; y1=416000; x2=348000; y2=416000; stroke=junction; }
					ha:line.16 { x1=332000; y1=436000; x2=356000; y2=436000; stroke=wire; }
					ha:line.17 { x1=356000; y1=436000; x2=356000; y2=416000; stroke=wire; }
					ha:line.18 { x1=356000; y1=416000; x2=364000; y2=416000; stroke=wire; }
					ha:line.19 { x1=348000; y1=436000; x2=348000; y2=436000; stroke=junction; }
					ha:line.20 { x1=356000; y1=416000; x2=356000; y2=416000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.745 {
				uuid=jsnwx40oi7YLGck+mCkAAAGs; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=364000; y=416000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAGt; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAGu; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:group.747 {
				uuid=jsnwx40oi7YLGck+mCkAAAGy; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=336000; y=484000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAGz; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAG0; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:connection.750 {
				li:conn {
					/2/731/1
					/2/724/3/1
				}
			}
			ha:connection.751 {
				li:conn {
					/2/737/5
					/2/724/4/1
				}
			}
			ha:connection.752 {
				li:conn {
					/2/724/6/1
					/2/737/16
				}
			}
			ha:group.772 {
				uuid=jsnwx40oi7YLGck+mCkAAAG+; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=360000; y=484000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAG/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAHA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:group.781 {
				uuid=jsnwx40oi7YLGck+mCkAAAHH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=364000; y=456000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAHI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAHJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=10k
				}
			}
			ha:group.783 {
				uuid=jsnwx40oi7YLGck+mCkAAAHK; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=364000; y=440000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAHL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAHM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=3k3
				}
			}
			ha:group.784 {
				uuid=jsnwx40oi7YLGck+mCkAAAHN;
				li:objects {
					ha:line.1 { x1=332000; y1=440000; x2=364000; y2=440000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.786 {
				li:conn {
					/2/784/1
					/2/724/5/1
				}
			}
			ha:group.794 {
				uuid=jsnwx40oi7YLGck+mCkAAAHQ;
				li:objects {
					ha:line.1 { x1=384000; y1=440000; x2=388000; y2=440000; stroke=wire; }
					ha:line.3 { x1=384000; y1=456000; x2=388000; y2=456000; stroke=wire; }
					ha:line.4 { x1=388000; y1=440000; x2=388000; y2=456000; stroke=wire; }
					ha:line.10 { x1=388000; y1=448000; x2=428000; y2=448000; stroke=wire; }
					ha:line.11 { x1=388000; y1=448000; x2=388000; y2=448000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.798 {
				uuid=jsnwx40oi7YLGck+mCkAAAHR;
				x=-20000; y=0;
				li:objects {
					ha:line.1 { x1=424000; y1=456000; x2=412000; y2=456000; stroke=wire; }
					ha:text.2 { x1=414000; y1=456000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND1
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.800 {
				uuid=jsnwx40oi7YLGck+mCkAAAHS;
				x=16000; y=0;
				li:objects {
					ha:line.1 { x1=248000; y1=452000; x2=228000; y2=452000; stroke=wire; }
					ha:text.2 { x1=228000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=IN_L_BUF
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.805 {
				uuid=jsnwx40oi7YLGck+mCkAAAHZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=404000; y=456000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAHa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAHb; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=20k
				}
			}
			ha:group.809 {
				uuid=jsnwx40oi7YLGck+mCkAAAHc;
				x=-20000; y=0;
				li:objects {
					ha:line.1 { x1=444000; y1=456000; x2=448000; y2=456000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.818 {
				uuid=jsnwx40oi7YLGck+mCkAAAHh; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=460000; y=452000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAHi; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAHj; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=10uF
				}
			}
			ha:group.824 {
				uuid=jsnwx40oi7YLGck+mCkAAAHr;
				x=-4000; y=0;
				li:objects {
					ha:line.2 { x1=484000; y1=452000; x2=496000; y2=452000; stroke=wire; }
					ha:text.3 { x1=488000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=L_BAX
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.851 {
				uuid=jsnwx40oi7YLGck+mCkAAAH2;
				li:objects {
					ha:line.1 { x1=336000; y1=488000; x2=336000; y2=488000; stroke=junction; }
					ha:line.3 { x1=348000; y1=460000; x2=332000; y2=460000; stroke=wire; }
					ha:line.4 { x1=336000; y1=484000; x2=336000; y2=488000; stroke=wire; }
					ha:line.5 { x1=312000; y1=488000; x2=348000; y2=488000; stroke=wire; }
					ha:line.6 { x1=348000; y1=460000; x2=348000; y2=488000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.852 {
				li:conn {
					/2/851/3
					/2/724/1/1
				}
			}
			ha:connection.853 {
				li:conn {
					/2/851/4
					/2/747/2/1
				}
			}
			ha:connection.857 {
				li:conn {
					/2/772/2/1
					/2/731/11
				}
			}
			ha:group.859 {
				uuid=jsnwx40oi7YLGck+mCkAAAH3;
				li:objects {
					ha:line.1 { x1=360000; y1=456000; x2=360000; y2=464000; stroke=wire; }
					ha:line.2 { x1=332000; y1=456000; x2=364000; y2=456000; stroke=wire; }
					ha:line.3 { x1=336000; y1=464000; x2=336000; y2=456000; stroke=wire; }
					ha:line.4 { x1=360000; y1=456000; x2=360000; y2=456000; stroke=junction; }
					ha:line.5 { x1=336000; y1=456000; x2=336000; y2=456000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.860 {
				li:conn {
					/2/859/1
					/2/772/1/1
				}
			}
			ha:connection.862 {
				li:conn {
					/2/859/2
					/2/724/2/1
				}
			}
			ha:connection.863 {
				li:conn {
					/2/859/3
					/2/747/1/1
				}
			}
			ha:connection.866 {
				li:conn {
					/2/723/1/1
					/2/737/6
				}
			}
			ha:connection.867 {
				li:conn {
					/2/723/2/1
					/2/880/3
				}
			}
			ha:connection.868 {
				li:conn {
					/2/721/2/1
					/2/880/4
				}
			}
			ha:connection.869 {
				li:conn {
					/2/851/5
					/2/721/1/1
				}
			}
			ha:connection.870 {
				li:conn {
					/2/781/2/1
					/2/859/2
				}
			}
			ha:connection.871 {
				li:conn {
					/2/783/2/1
					/2/784/1
				}
			}
			ha:connection.872 {
				li:conn {
					/2/781/1/1
					/2/794/3
				}
			}
			ha:connection.873 {
				li:conn {
					/2/783/1/1
					/2/794/1
				}
			}
			ha:connection.875 {
				li:conn {
					/2/720/2/1
					/2/794/10
				}
			}
			ha:connection.876 {
				li:conn {
					/2/745/2/1
					/2/737/18
				}
			}
			ha:connection.877 {
				li:conn {
					/2/733/2/1
					/2/731/12
				}
			}
			ha:connection.879 {
				li:conn {
					/2/745/1/1
					/2/905/6
				}
			}
			ha:group.880 {
				uuid=jsnwx40oi7YLGck+mCkAAAH5;
				x=16000; y=0;
				li:objects {
					ha:line.1 { x1=268000; y1=452000; x2=272000; y2=452000; stroke=wire; }
					ha:line.2 { x1=272000; y1=416000; x2=272000; y2=488000; stroke=wire; }
					ha:line.3 { x1=272000; y1=416000; x2=276000; y2=416000; stroke=wire; }
					ha:line.4 { x1=272000; y1=488000; x2=276000; y2=488000; stroke=wire; }
					ha:line.5 { x1=272000; y1=452000; x2=272000; y2=452000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.883 {
				li:conn {
					/2/800/1
					/2/722/2/1
				}
			}
			ha:connection.884 {
				li:conn {
					/2/880/1
					/2/722/1/1
				}
			}
			ha:connection.897 {
				li:conn {
					/2/809/1
					/2/720/1/1
				}
			}
			ha:connection.902 {
				li:conn {
					/2/805/2/1
					/2/798/1
				}
			}
			ha:connection.903 {
				li:conn {
					/2/809/1
					/2/805/1/1
				}
			}
			ha:group.905 {
				uuid=jsnwx40oi7YLGck+mCkAAAH6;
				li:objects {
					ha:line.1 { x1=456000; y1=416000; x2=456000; y2=488000; stroke=wire; }
					ha:line.2 { x1=456000; y1=488000; x2=384000; y2=488000; stroke=wire; }
					ha:line.3 { x1=452000; y1=452000; x2=460000; y2=452000; stroke=wire; }
					ha:line.5 { x1=456000; y1=452000; x2=456000; y2=452000; stroke=junction; }
					ha:line.6 { x1=456000; y1=416000; x2=384000; y2=416000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.906 {
				li:conn {
					/2/905/2
					/2/733/1/1
				}
			}
			ha:connection.907 {
				li:conn {
					/2/905/3
					/2/720/3/1
				}
			}
			ha:connection.915 {
				li:conn {
					/2/818/2/1
					/2/905/3
				}
			}
			ha:group.920 {
				uuid=jsnwx40oi7YLGck+mCkAAAH7;
				x=40000; y=368000;
				li:objects {
					ha:line.1 { x1=124000; y1=220000; x2=148000; y2=220000; stroke=wire; }
					ha:line.2 { x1=128000; y1=220000; x2=128000; y2=196000; stroke=wire; }
					ha:line.3 { x1=128000; y1=196000; x2=124000; y2=196000; stroke=wire; }
					ha:line.4 { x1=128000; y1=220000; x2=128000; y2=220000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.924 {
				li:conn {
					/2/580/1/1
					/2/644/10
				}
			}
			ha:group.925 {
				uuid=jsnwx40oi7YLGck+mCkAAAH9;
				li:objects {
					ha:line.1 { x1=188000; y1=132000; x2=188000; y2=116000; stroke=wire; }
					ha:line.5 { x1=184000; y1=132000; x2=188000; y2=132000; stroke=wire; }
					ha:line.6 { x1=184000; y1=116000; x2=196000; y2=116000; stroke=wire; }
					ha:line.7 { x1=188000; y1=116000; x2=188000; y2=116000; stroke=junction; }
					ha:text.9 { x1=190000; y1=116000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=R_BAX
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.928 {
				li:conn {
					/2/925/5
					/2/348/1/1
				}
			}
			ha:connection.929 {
				li:conn {
					/2/925/6
					/2/348/4/1
				}
			}
			ha:group.930 {
				uuid=jsnwx40oi7YLGck+mCkAAAH+;
				x=40000; y=368000;
				li:objects {
					ha:line.1 { x1=128000; y1=144000; x2=120000; y2=144000; stroke=wire; }
					ha:line.2 { x1=128000; y1=144000; x2=128000; y2=168000; stroke=wire; }
					ha:line.3 { x1=124000; y1=168000; x2=148000; y2=168000; stroke=wire; }
					ha:line.4 { x1=128000; y1=168000; x2=128000; y2=168000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.966 {
				li:conn {
					/2/52/1
					/2/8/3/1
				}
			}
			ha:connection.967 {
				li:conn {
					/2/52/3
					/2/51/12/1
				}
			}
			ha:connection.968 {
				li:conn {
					/2/55/1
					/2/8/2/1
				}
			}
			ha:connection.969 {
				li:conn {
					/2/55/3
					/2/51/11/1
				}
			}
			ha:connection.970 {
				li:conn {
					/2/59/1
					/2/9/3/1
				}
			}
			ha:connection.971 {
				li:conn {
					/2/59/3
					/2/58/12/1
				}
			}
			ha:connection.972 {
				li:conn {
					/2/62/1
					/2/9/2/1
				}
			}
			ha:connection.973 {
				li:conn {
					/2/62/3
					/2/58/11/1
				}
			}
			ha:connection.974 {
				li:conn {
					/2/66/1
					/2/6/2/1
				}
			}
			ha:connection.975 {
				li:conn {
					/2/66/1
					/2/51/9/1
				}
			}
			ha:connection.976 {
				li:conn {
					/2/69/1
					/2/65/2/1
				}
			}
			ha:connection.977 {
				li:conn {
					/2/69/1
					/2/58/9/1
				}
			}
			ha:connection.978 {
				li:conn {
					/2/88/1
					/2/51/10/1
				}
			}
			ha:connection.979 {
				li:conn {
					/2/88/2
					/2/72/1/1
				}
			}
			ha:connection.980 {
				li:conn {
					/2/91/1
					/2/73/1/1
				}
			}
			ha:connection.981 {
				li:conn {
					/2/91/2
					/2/58/10/1
				}
			}
			ha:connection.982 {
				li:conn {
					/2/94/1
					/2/6/1/1
				}
			}
			ha:connection.983 {
				li:conn {
					/2/94/1
					/2/7/2/1
				}
			}
			ha:connection.984 {
				li:conn {
					/2/98/1
					/2/65/1/1
				}
			}
			ha:connection.985 {
				li:conn {
					/2/98/1
					/2/97/2/1
				}
			}
			ha:connection.986 {
				li:conn {
					/2/108/1
					/2/104/2/1
				}
			}
			ha:connection.987 {
				li:conn {
					/2/108/2
					/2/3/2/1
				}
			}
			ha:connection.988 {
				li:conn {
					/2/108/2
					/2/97/1/1
				}
			}
			ha:connection.989 {
				li:conn {
					/2/127/1
					/2/126/2/1
				}
			}
			ha:connection.990 {
				li:conn {
					/2/127/2
					/2/2/2/1
				}
			}
			ha:connection.991 {
				li:conn {
					/2/127/2
					/2/7/1/1
				}
			}
			ha:connection.992 {
				li:conn {
					/2/132/1
					/2/3/1/1
				}
			}
			ha:connection.993 {
				li:conn {
					/2/134/1
					/2/2/1/1
				}
			}
			ha:connection.994 {
				li:conn {
					/2/196/1
					/2/2/11/1
				}
			}
			ha:connection.995 {
				li:conn {
					/2/200/1
					/2/2/10/1
				}
			}
			ha:connection.996 {
				li:conn {
					/2/200/1
					/2/199/1/1
				}
			}
			ha:connection.997 {
				li:conn {
					/2/204/1
					/2/3/11/1
				}
			}
			ha:connection.998 {
				li:conn {
					/2/204/1
					/2/203/1/1
				}
			}
			ha:connection.999 {
				li:conn {
					/2/208/1
					/2/3/10/1
				}
			}
			ha:connection.1000 {
				li:conn {
					/2/208/1
					/2/207/1/1
				}
			}
			ha:connection.1001 {
				li:conn {
					/2/449/1/1
					/2/196/1
				}
			}
			ha:connection.1002 {
				li:conn {
					/2/920/1
					/2/2/3/1
				}
			}
			ha:connection.1003 {
				li:conn {
					/2/920/3
					/2/126/1/1
				}
			}
			ha:connection.1004 {
				li:conn {
					/2/930/1
					/2/104/1/1
				}
			}
			ha:connection.1005 {
				li:conn {
					/2/930/3
					/2/3/3/1
				}
			}
			ha:connection.1012 {
				li:conn {
					/2/824/2
					/2/818/1/1
				}
			}
			ha:group.1013 {
				uuid=jsnwx40oi7YLGck+mCkAAAJB; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAH;
				x=452000; y=360000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAI;
						x=-20000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in+
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJD; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAJ;
						x=-20000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=in-
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jsnwx40oi7YLGck+mCkAAAJE; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAK;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=out
							role=terminal
						}
					}
					ha:line.4 { x1=-20000; y1=-8000; x2=-20000; y2=8000; stroke=sym-decor; }
					ha:line.5 { x1=-20000; y1=8000; x2=-4000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-4000; y1=0; x2=-20000; y2=-8000; stroke=sym-decor; }
					ha:line.7 { x1=-18000; y1=5000; x2=-18000; y2=3000; stroke=sym-decor; }
					ha:line.8 { x1=-19000; y1=4000; x2=-17000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=-19000; y1=-4000; x2=-17000; y2=-4000; stroke=sym-decor; }
					ha:group.10 {
						uuid=jsnwx40oi7YLGck+mCkAAAJF; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAL;
						x=-12000; y=-4000; rot=270.000000; mirx=1; miry=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=0; y1=-1000; rot=180.000000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V-
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jsnwx40oi7YLGck+mCkAAAJG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAM;
						x=-12000; y=8000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=-4000; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=V+
							role=terminal
						}
					}
					ha:text.12 { x1=-21000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-slot=3
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=quad_opamp_so14
					name=U?
					role=symbol
				}
			}
			ha:group.1014 {
				uuid=jsnwx40oi7YLGck+mCkAAAJH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=292000; y=396000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=10k
				}
			}
			ha:group.1015 {
				uuid=jsnwx40oi7YLGck+mCkAAAJK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=264000; y=360000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJM; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=10uF
				}
			}
			ha:group.1016 {
				uuid=jsnwx40oi7YLGck+mCkAAAJN; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=292000; y=324000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJO; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJP; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:group.1017 {
				uuid=jsnwx40oi7YLGck+mCkAAAJQ; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQ3;
				x=324000; y=376000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJR; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAP/;
						x=8000; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0B
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJS; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQA;
						x=8000; y=-12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W0
							role=terminal
						}
					}
					ha:group.3 {
						uuid=jsnwx40oi7YLGck+mCkAAAJT; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQB;
						x=8000; y=-16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P0A
							role=terminal
						}
					}
					ha:group.4 {
						uuid=jsnwx40oi7YLGck+mCkAAAJU; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQC;
						x=8000; y=-24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1B
							role=terminal
						}
					}
					ha:group.5 {
						uuid=jsnwx40oi7YLGck+mCkAAAJV; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQD;
						x=8000; y=-28000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=W1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=jsnwx40oi7YLGck+mCkAAAJW; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQE;
						x=8000; y=-32000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P1A
							role=terminal
						}
					}
					ha:group.7 {
						uuid=jsnwx40oi7YLGck+mCkAAAJX; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQF;
						x=-16000; y=-8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SI
							role=terminal
						}
					}
					ha:group.8 {
						uuid=jsnwx40oi7YLGck+mCkAAAJY; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQG;
						x=-16000; y=-12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SO
							role=terminal
						}
					}
					ha:group.9 {
						uuid=jsnwx40oi7YLGck+mCkAAAJZ; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQH;
						x=-16000; y=-16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=SCK
							role=terminal
						}
					}
					ha:group.10 {
						uuid=jsnwx40oi7YLGck+mCkAAAJa; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQI;
						x=-16000; y=-20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={CS#}
							role=terminal
						}
					}
					ha:group.11 {
						uuid=jsnwx40oi7YLGck+mCkAAAJb; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQJ;
						x=-16000; y=-24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={RS#}
							role=terminal
						}
					}
					ha:group.12 {
						uuid=jsnwx40oi7YLGck+mCkAAAJc; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQK;
						x=-16000; y=-28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name={SHDN#}
							role=terminal
						}
					}
					ha:group.13 {
						uuid=jsnwx40oi7YLGck+mCkAAAJd; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQL;
						x=-4000; y=0; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VDD
							role=terminal
						}
					}
					ha:group.14 {
						uuid=jsnwx40oi7YLGck+mCkAAAJe; src_uuid=Mn6K7gaZfMoPBT2+M4YAAAQM;
						x=-4000; y=-40000; rot=270.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=VSS
							role=terminal
						}
					}
					ha:line.15 { x1=4000; y1=-8000; x2=0; y2=-8000; stroke=sym-decor; }
					ha:line.16 { x1=4000; y1=-16000; x2=0; y2=-16000; stroke=sym-decor; }
					ha:line.17 { x1=0; y1=-8000; x2=0; y2=-9000; stroke=sym-decor; }
					ha:line.18 { x1=0; y1=-16000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.19 { x1=0; y1=-9000; x2=1000; y2=-10000; stroke=sym-decor; }
					ha:line.20 { x1=1000; y1=-10000; x2=-1000; y2=-11000; stroke=sym-decor; }
					ha:line.21 { x1=-1000; y1=-11000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.22 { x1=1000; y1=-12000; x2=-1000; y2=-13000; stroke=sym-decor; }
					ha:line.23 { x1=-1000; y1=-13000; x2=1000; y2=-14000; stroke=sym-decor; }
					ha:line.24 { x1=1000; y1=-14000; x2=0; y2=-15000; stroke=sym-decor; }
					ha:line.25 { x1=4000; y1=-12000; x2=1000; y2=-12000; stroke=sym-decor; }
					ha:line.26 { x1=1000; y1=-12000; x2=3000; y2=-11000; stroke=sym-decor; }
					ha:line.27 { x1=1000; y1=-12000; x2=3000; y2=-13000; stroke=sym-decor; }
					ha:line.28 { x1=1000; y1=-28000; x2=-1000; y2=-29000; stroke=sym-decor; }
					ha:line.29 { x1=-1000; y1=-29000; x2=1000; y2=-30000; stroke=sym-decor; }
					ha:line.30 { x1=1000; y1=-30000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.31 { x1=4000; y1=-28000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:line.32 { x1=1000; y1=-28000; x2=3000; y2=-27000; stroke=sym-decor; }
					ha:line.33 { x1=1000; y1=-28000; x2=3000; y2=-29000; stroke=sym-decor; }
					ha:line.34 { x1=4000; y1=-24000; x2=0; y2=-24000; stroke=sym-decor; }
					ha:line.35 { x1=4000; y1=-32000; x2=0; y2=-32000; stroke=sym-decor; }
					ha:line.36 { x1=0; y1=-24000; x2=0; y2=-25000; stroke=sym-decor; }
					ha:line.37 { x1=0; y1=-32000; x2=0; y2=-31000; stroke=sym-decor; }
					ha:line.38 { x1=0; y1=-25000; x2=1000; y2=-26000; stroke=sym-decor; }
					ha:text.39 { x1=-10000; y1=-8000; dyntext=0; stroke=sym-decor; text=SI; }
					ha:text.40 { x1=-10000; y1=-12000; dyntext=0; stroke=sym-decor; text=SO; }
					ha:text.41 { x1=-10000; y1=-16000; dyntext=0; stroke=sym-decor; text=SCK; }
					ha:line.42 { x1=1000; y1=-26000; x2=-1000; y2=-27000; stroke=sym-decor; }
					ha:line.43 { x1=-1000; y1=-27000; x2=1000; y2=-28000; stroke=sym-decor; }
					ha:polygon.44 {
						li:outline {
							ha:line { x1=-12000; y1=-4000; x2=-12000; y2=-36000; }
							ha:line { x1=-12000; y1=-36000; x2=4000; y2=-36000; }
							ha:line { x1=4000; y1=-36000; x2=4000; y2=-4000; }
							ha:line { x1=4000; y1=-4000; x2=-12000; y2=-4000; }
						}
						stroke=sym-decor;
					}
					ha:text.45 { x1=0; y1=-8000; dyntext=0; stroke=sym-decor; text=0B; }
					ha:text.46 { x1=0; y1=-19000; dyntext=0; stroke=sym-decor; text=0A; }
					ha:text.47 { x1=0; y1=-24000; dyntext=0; stroke=sym-decor; text=1B; }
					ha:text.48 { x1=-10000; y1=-20000; dyntext=0; stroke=sym-decor; text=CS; }
					ha:text.49 { x1=-10000; y1=-24000; dyntext=0; stroke=sym-decor; text=RS; }
					ha:text.50 { x1=-10000; y1=-28000; dyntext=0; stroke=sym-decor; text=SHDN; }
					ha:text.51 { x1=0; y1=-35000; dyntext=0; stroke=sym-decor; text=1A; }
					ha:text.52 { x1=-2000; y1=-9000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VDD; }
					ha:text.53 { x1=-2000; y1=-35000; rot=90.000000; dyntext=0; stroke=sym-decor; text=VSS; }
					ha:text.54 { x1=-2000; y1=-3000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=U?
					role=symbol
					value=100k
				}
			}
			ha:group.1018 {
				uuid=jsnwx40oi7YLGck+mCkAAAJf; src_uuid=jsnwx40oi7YLGck+mCkAAAGd;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=332000; y1=452000; x2=352000; y2=452000; stroke=wire; }
					ha:line.7 { x1=352000; y1=452000; x2=352000; y2=488000; stroke=wire; }
					ha:line.11 { x1=360000; y1=484000; x2=360000; y2=488000; stroke=wire; }
					ha:line.12 { x1=352000; y1=488000; x2=364000; y2=488000; stroke=wire; }
					ha:line.15 { x1=360000; y1=488000; x2=360000; y2=488000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1019 {
				li:conn {
					/2/1018/1
					/2/1017/3/1
				}
			}
			ha:group.1020 {
				uuid=jsnwx40oi7YLGck+mCkAAAJg; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=364000; y=396000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJh; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJi; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=10k
				}
			}
			ha:connection.1021 {
				li:conn {
					/2/1020/2/1
					/2/1018/12
				}
			}
			ha:group.1022 {
				uuid=jsnwx40oi7YLGck+mCkAAAJj; src_uuid=jsnwx40oi7YLGck+mCkAAAGl;
				x=0; y=-92000;
				li:objects {
					ha:line.4 { x1=348000; y1=416000; x2=348000; y2=444000; stroke=wire; }
					ha:line.5 { x1=332000; y1=444000; x2=348000; y2=444000; stroke=wire; }
					ha:line.6 { x1=312000; y1=416000; x2=360000; y2=416000; stroke=wire; }
					ha:line.15 { x1=348000; y1=416000; x2=348000; y2=416000; stroke=junction; }
					ha:line.16 { x1=332000; y1=436000; x2=356000; y2=436000; stroke=wire; }
					ha:line.17 { x1=356000; y1=436000; x2=356000; y2=416000; stroke=wire; }
					ha:line.18 { x1=356000; y1=416000; x2=364000; y2=416000; stroke=wire; }
					ha:line.19 { x1=348000; y1=436000; x2=348000; y2=436000; stroke=junction; }
					ha:line.20 { x1=356000; y1=416000; x2=356000; y2=416000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1023 {
				li:conn {
					/2/1022/5
					/2/1017/4/1
				}
			}
			ha:connection.1024 {
				li:conn {
					/2/1022/6
					/2/1016/1/1
				}
			}
			ha:connection.1025 {
				li:conn {
					/2/1022/16
					/2/1017/6/1
				}
			}
			ha:group.1026 {
				uuid=jsnwx40oi7YLGck+mCkAAAJk; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=364000; y=324000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJl; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJm; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:connection.1027 {
				li:conn {
					/2/1026/2/1
					/2/1022/18
				}
			}
			ha:group.1028 {
				uuid=jsnwx40oi7YLGck+mCkAAAJn; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=336000; y=392000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJo; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJp; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:group.1029 {
				uuid=jsnwx40oi7YLGck+mCkAAAJq; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=360000; y=392000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJr; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJs; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=68nF
				}
			}
			ha:connection.1030 {
				li:conn {
					/2/1029/2/1
					/2/1018/11
				}
			}
			ha:group.1031 {
				uuid=jsnwx40oi7YLGck+mCkAAAJt; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=364000; y=364000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJu; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJv; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=10k
				}
			}
			ha:group.1032 {
				uuid=jsnwx40oi7YLGck+mCkAAAJw; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=364000; y=348000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJx; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJy; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=3k3
				}
			}
			ha:group.1033 {
				uuid=jsnwx40oi7YLGck+mCkAAAJz; src_uuid=jsnwx40oi7YLGck+mCkAAAHN;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=332000; y1=440000; x2=364000; y2=440000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1034 {
				li:conn {
					/2/1033/1
					/2/1032/2/1
				}
			}
			ha:connection.1035 {
				li:conn {
					/2/1033/1
					/2/1017/5/1
				}
			}
			ha:group.1040 {
				uuid=jsnwx40oi7YLGck+mCkAAAJ1; src_uuid=jsnwx40oi7YLGck+mCkAAAHR;
				x=-20000; y=-92000;
				li:objects {
					ha:line.1 { x1=424000; y1=456000; x2=412000; y2=456000; stroke=wire; }
					ha:text.2 { x1=414000; y1=456000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=VGND2
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.1041 {
				uuid=jsnwx40oi7YLGck+mCkAAAJ2; src_uuid=jsnwx40oi7YLGck+mCkAAAHS;
				x=16000; y=-92000;
				li:objects {
					ha:line.1 { x1=248000; y1=452000; x2=228000; y2=452000; stroke=wire; }
					ha:text.2 { x1=228000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=IN_R_BUF
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1042 {
				li:conn {
					/2/1041/1
					/2/1015/2/1
				}
			}
			ha:group.1043 {
				uuid=jsnwx40oi7YLGck+mCkAAAJ3; src_uuid=iNOQfJpO6hT/HFDFGjoAAABI;
				x=404000; y=364000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJ4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABJ;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJ5; src_uuid=iNOQfJpO6hT/HFDFGjoAAABK;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=14000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=10000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=2000; x2=7000; y2=-2000; stroke=sym-decor; }
					ha:line.6 { x1=7000; y1=-2000; x2=5000; y2=2000; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=-2000; x2=13000; y2=2000; stroke=sym-decor; }
					ha:line.9 { x1=13000; y1=2000; x2=15000; y2=-2000; stroke=sym-decor; }
					ha:line.10 { x1=15000; y1=-2000; x2=16000; y2=0; stroke=sym-decor; }
					ha:line.11 { x1=5000; y1=2000; x2=4000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=R??
					role=symbol
					value=20k
				}
			}
			ha:connection.1044 {
				li:conn {
					/2/1043/2/1
					/2/1040/1
				}
			}
			ha:group.1045 {
				uuid=jsnwx40oi7YLGck+mCkAAAJ6; src_uuid=jsnwx40oi7YLGck+mCkAAAHc;
				x=-20000; y=-92000;
				li:objects {
					ha:line.1 { x1=444000; y1=456000; x2=448000; y2=456000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1046 {
				li:conn {
					/2/1045/1
					/2/1013/1/1
				}
			}
			ha:connection.1047 {
				li:conn {
					/2/1045/1
					/2/1043/1/1
				}
			}
			ha:group.1048 {
				uuid=jsnwx40oi7YLGck+mCkAAAJ7; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=460000; y=360000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAJ8; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=jsnwx40oi7YLGck+mCkAAAJ9; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					name=C??
					role=symbol
					value=10uF
				}
			}
			ha:group.1049 {
				uuid=jsnwx40oi7YLGck+mCkAAAJ+; src_uuid=jsnwx40oi7YLGck+mCkAAAHr;
				x=-4000; y=-92000;
				li:objects {
					ha:line.2 { x1=484000; y1=452000; x2=496000; y2=452000; stroke=wire; }
					ha:text.3 { x1=488000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=R_BAX
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1050 {
				li:conn {
					/2/1049/2
					/2/1048/1/1
				}
			}
			ha:group.1055 {
				uuid=jsnwx40oi7YLGck+mCkAAAKA; src_uuid=jsnwx40oi7YLGck+mCkAAAH3;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=360000; y1=456000; x2=360000; y2=464000; stroke=wire; }
					ha:line.2 { x1=332000; y1=456000; x2=364000; y2=456000; stroke=wire; }
					ha:line.3 { x1=336000; y1=464000; x2=336000; y2=456000; stroke=wire; }
					ha:line.4 { x1=360000; y1=456000; x2=360000; y2=456000; stroke=junction; }
					ha:line.5 { x1=336000; y1=456000; x2=336000; y2=456000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1056 {
				li:conn {
					/2/1055/1
					/2/1029/1/1
				}
			}
			ha:connection.1057 {
				li:conn {
					/2/1055/2
					/2/1017/2/1
				}
			}
			ha:connection.1058 {
				li:conn {
					/2/1055/2
					/2/1031/2/1
				}
			}
			ha:connection.1059 {
				li:conn {
					/2/1055/3
					/2/1028/1/1
				}
			}
			ha:group.1060 {
				uuid=jsnwx40oi7YLGck+mCkAAAKB; src_uuid=jsnwx40oi7YLGck+mCkAAAH5;
				x=16000; y=-92000;
				li:objects {
					ha:line.1 { x1=268000; y1=452000; x2=272000; y2=452000; stroke=wire; }
					ha:line.2 { x1=272000; y1=416000; x2=272000; y2=488000; stroke=wire; }
					ha:line.3 { x1=272000; y1=416000; x2=276000; y2=416000; stroke=wire; }
					ha:line.4 { x1=272000; y1=488000; x2=276000; y2=488000; stroke=wire; }
					ha:line.5 { x1=272000; y1=452000; x2=272000; y2=452000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1061 {
				li:conn {
					/2/1060/1
					/2/1015/1/1
				}
			}
			ha:connection.1062 {
				li:conn {
					/2/1060/3
					/2/1016/2/1
				}
			}
			ha:connection.1063 {
				li:conn {
					/2/1060/4
					/2/1014/2/1
				}
			}
			ha:group.1064 {
				uuid=jsnwx40oi7YLGck+mCkAAAKC; src_uuid=jsnwx40oi7YLGck+mCkAAAH6;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=456000; y1=416000; x2=456000; y2=488000; stroke=wire; }
					ha:line.2 { x1=456000; y1=488000; x2=384000; y2=488000; stroke=wire; }
					ha:line.3 { x1=452000; y1=452000; x2=460000; y2=452000; stroke=wire; }
					ha:line.5 { x1=456000; y1=452000; x2=456000; y2=452000; stroke=junction; }
					ha:line.6 { x1=456000; y1=416000; x2=384000; y2=416000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1065 {
				li:conn {
					/2/1064/2
					/2/1020/1/1
				}
			}
			ha:connection.1066 {
				li:conn {
					/2/1064/3
					/2/1048/2/1
				}
			}
			ha:connection.1067 {
				li:conn {
					/2/1064/3
					/2/1013/3/1
				}
			}
			ha:connection.1068 {
				li:conn {
					/2/1064/6
					/2/1026/1/1
				}
			}
			ha:group.1069 {
				uuid=jsnwx40oi7YLGck+mCkAAAKD;
				li:objects {
					ha:line.1 { x1=388000; y1=348000; x2=388000; y2=364000; stroke=wire; }
					ha:line.2 { x1=388000; y1=356000; x2=428000; y2=356000; stroke=wire; }
					ha:line.3 { x1=384000; y1=348000; x2=388000; y2=348000; stroke=wire; }
					ha:line.4 { x1=388000; y1=356000; x2=388000; y2=356000; stroke=junction; }
					ha:line.5 { x1=384000; y1=364000; x2=388000; y2=364000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1070 {
				li:conn {
					/2/1069/2
					/2/1013/2/1
				}
			}
			ha:connection.1071 {
				li:conn {
					/2/1069/3
					/2/1032/1/1
				}
			}
			ha:connection.1072 {
				li:conn {
					/2/1069/5
					/2/1031/1/1
				}
			}
			ha:group.1073 {
				uuid=jsnwx40oi7YLGck+mCkAAAKE;
				li:objects {
					ha:line.1 { x1=348000; y1=368000; x2=348000; y2=396000; stroke=wire; }
					ha:line.2 { x1=336000; y1=392000; x2=336000; y2=396000; stroke=wire; }
					ha:line.3 { x1=348000; y1=368000; x2=332000; y2=368000; stroke=wire; }
					ha:line.4 { x1=312000; y1=396000; x2=348000; y2=396000; stroke=wire; }
					ha:line.5 { x1=336000; y1=396000; x2=336000; y2=396000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1074 {
				li:conn {
					/2/1073/2
					/2/1028/2/1
				}
			}
			ha:connection.1075 {
				li:conn {
					/2/1073/3
					/2/1017/1/1
				}
			}
			ha:connection.1076 {
				li:conn {
					/2/1073/4
					/2/1014/1/1
				}
			}
			ha:group.1077 {
				uuid=jsnwx40oi7YLGck+mCkAAAKH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=440000; y=464000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKI; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.1078 {
				uuid=jsnwx40oi7YLGck+mCkAAAKJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=440000; y=372000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.1079 {
				uuid=jsnwx40oi7YLGck+mCkAAAKN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=440000; y=440000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.1080 {
				uuid=jsnwx40oi7YLGck+mCkAAAKP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=440000; y=348000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.1081 {
				uuid=jsnwx40oi7YLGck+mCkAAAKT; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB8;
				x=320000; y=384000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKU; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB9;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vdd; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vdd}
					}
					role=symbol
				}
			}
			ha:group.1082 {
				uuid=jsnwx40oi7YLGck+mCkAAAKV; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB8;
				x=320000; y=476000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKW; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB9;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vdd; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vdd}
					}
					role=symbol
				}
			}
			ha:group.1083 {
				uuid=jsnwx40oi7YLGck+mCkAAAKZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAACA;
				x=320000; y=332000; rot=180.000000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKa; src_uuid=iNOQfJpO6hT/HFDFGjoAAACB;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vss; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vss}
					}
					role=symbol
				}
			}
			ha:group.1084 {
				uuid=jsnwx40oi7YLGck+mCkAAAKb; src_uuid=iNOQfJpO6hT/HFDFGjoAAACA;
				x=320000; y=424000; rot=180.000000;
				li:objects {
					ha:group.1 {
						uuid=jsnwx40oi7YLGck+mCkAAAKc; src_uuid=iNOQfJpO6hT/HFDFGjoAAACB;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vss; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vss}
					}
					role=symbol
				}
			}
			ha:group.1085 {
				uuid=jsnwx40oi7YLGck+mCkAAAKd;
				li:objects {
					ha:line.1 { x1=320000; y1=424000; x2=320000; y2=428000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1086 {
				li:conn {
					/2/1085/1
					/2/724/14/1
				}
			}
			ha:connection.1087 {
				li:conn {
					/2/1085/1
					/2/1084/1/1
				}
			}
			ha:group.1088 {
				uuid=jsnwx40oi7YLGck+mCkAAAKe;
				li:objects {
					ha:line.1 { x1=320000; y1=332000; x2=320000; y2=336000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1089 {
				li:conn {
					/2/1088/1
					/2/1017/14/1
				}
			}
			ha:connection.1090 {
				li:conn {
					/2/1088/1
					/2/1083/1/1
				}
			}
			ha:group.1097 {
				uuid=jsnwx40oi7YLGck+mCkAAAKh;
				li:objects {
					ha:line.1 { x1=440000; y1=460000; x2=440000; y2=464000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1098 {
				li:conn {
					/2/1097/1
					/2/720/11/1
				}
			}
			ha:connection.1099 {
				li:conn {
					/2/1097/1
					/2/1077/1/1
				}
			}
			ha:group.1100 {
				uuid=jsnwx40oi7YLGck+mCkAAAKi;
				li:objects {
					ha:line.1 { x1=440000; y1=440000; x2=440000; y2=444000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1101 {
				li:conn {
					/2/1100/1
					/2/720/10/1
				}
			}
			ha:connection.1102 {
				li:conn {
					/2/1100/1
					/2/1079/1/1
				}
			}
			ha:group.1103 {
				uuid=jsnwx40oi7YLGck+mCkAAAKj;
				li:objects {
					ha:line.1 { x1=440000; y1=368000; x2=440000; y2=372000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1104 {
				li:conn {
					/2/1103/1
					/2/1078/1/1
				}
			}
			ha:connection.1105 {
				li:conn {
					/2/1103/1
					/2/1013/11/1
				}
			}
			ha:group.1106 {
				uuid=jsnwx40oi7YLGck+mCkAAAKk;
				li:objects {
					ha:line.1 { x1=440000; y1=348000; x2=440000; y2=352000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1107 {
				li:conn {
					/2/1106/1
					/2/1080/1/1
				}
			}
			ha:connection.1108 {
				li:conn {
					/2/1106/1
					/2/1013/10/1
				}
			}
			ha:group.1109 {
				uuid=jsnwx40oi7YLGck+mCkAAAKl;
				li:objects {
					ha:line.1 { x1=296000; y1=460000; x2=308000; y2=460000; stroke=wire; }
					ha:text.2 { x1=296000; y1=460000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1110 {
				li:conn {
					/2/1109/1
					/2/724/7/1
				}
			}
			ha:group.1111 {
				uuid=jsnwx40oi7YLGck+mCkAAAKm;
				li:objects {
					ha:line.1 { x1=296000; y1=456000; x2=308000; y2=456000; stroke=wire; }
					ha:text.2 { x1=296000; y1=456000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1112 {
				li:conn {
					/2/1111/1
					/2/724/8/1
				}
			}
			ha:group.1113 {
				uuid=jsnwx40oi7YLGck+mCkAAAKn;
				li:objects {
					ha:line.1 { x1=296000; y1=452000; x2=308000; y2=452000; stroke=wire; }
					ha:text.2 { x1=296000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1114 {
				li:conn {
					/2/1113/1
					/2/724/9/1
				}
			}
			ha:group.1115 {
				uuid=jsnwx40oi7YLGck+mCkAAAKo;
				li:objects {
					ha:line.1 { x1=296000; y1=448000; x2=308000; y2=448000; stroke=wire; }
					ha:text.2 { x1=296000; y1=448000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CS_BXL
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1116 {
				li:conn {
					/2/1115/1
					/2/724/10/1
				}
			}
			ha:group.1117 {
				uuid=jsnwx40oi7YLGck+mCkAAAKt; src_uuid=jsnwx40oi7YLGck+mCkAAAKo;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=296000; y1=448000; x2=308000; y2=448000; stroke=wire; }
					ha:text.2 { x1=296000; y1=448000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CS_BXR
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1118 {
				li:conn {
					/2/1117/1
					/2/1017/10/1
				}
			}
			ha:group.1119 {
				uuid=jsnwx40oi7YLGck+mCkAAAKu; src_uuid=jsnwx40oi7YLGck+mCkAAAKl;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=296000; y1=460000; x2=308000; y2=460000; stroke=wire; }
					ha:text.2 { x1=296000; y1=460000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1120 {
				li:conn {
					/2/1119/1
					/2/1017/7/1
				}
			}
			ha:group.1121 {
				uuid=jsnwx40oi7YLGck+mCkAAAKv; src_uuid=jsnwx40oi7YLGck+mCkAAAKm;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=296000; y1=456000; x2=308000; y2=456000; stroke=wire; }
					ha:text.2 { x1=296000; y1=456000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1122 {
				li:conn {
					/2/1121/1
					/2/1017/8/1
				}
			}
			ha:group.1123 {
				uuid=jsnwx40oi7YLGck+mCkAAAKw; src_uuid=jsnwx40oi7YLGck+mCkAAAKn;
				x=0; y=-92000;
				li:objects {
					ha:line.1 { x1=296000; y1=452000; x2=308000; y2=452000; stroke=wire; }
					ha:text.2 { x1=296000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1124 {
				li:conn {
					/2/1123/1
					/2/1017/9/1
				}
			}
			ha:group.1129 {
				uuid=jsnwx40oi7YLGck+mCkAAAKz;
				li:objects {
					ha:line.1 { x1=292000; y1=380000; x2=320000; y2=380000; stroke=wire; }
					ha:line.2 { x1=292000; y1=348000; x2=292000; y2=380000; stroke=wire; }
					ha:line.3 { x1=292000; y1=352000; x2=292000; y2=352000; stroke=junction; }
					ha:line.4 { x1=308000; y1=348000; x2=292000; y2=348000; stroke=wire; }
					ha:line.5 { x1=308000; y1=352000; x2=292000; y2=352000; stroke=wire; }
					ha:line.7 { x1=320000; y1=376000; x2=320000; y2=384000; stroke=wire; }
					ha:line.8 { x1=320000; y1=380000; x2=320000; y2=380000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1131 {
				li:conn {
					/2/1129/4
					/2/1017/12/1
				}
			}
			ha:connection.1132 {
				li:conn {
					/2/1129/5
					/2/1017/11/1
				}
			}
			ha:connection.1133 {
				li:conn {
					/2/1017/13/1
					/2/1129/7
				}
			}
			ha:group.1138 {
				uuid=jsnwx40oi7YLGck+mCkAAAK2;
				li:objects {
					ha:line.1 { x1=308000; y1=440000; x2=292000; y2=440000; stroke=wire; }
					ha:line.2 { x1=292000; y1=440000; x2=292000; y2=472000; stroke=wire; }
					ha:line.3 { x1=308000; y1=444000; x2=292000; y2=444000; stroke=wire; }
					ha:line.5 { x1=292000; y1=472000; x2=320000; y2=472000; stroke=wire; }
					ha:line.6 { x1=292000; y1=444000; x2=292000; y2=444000; stroke=junction; }
					ha:line.7 { x1=320000; y1=468000; x2=320000; y2=476000; stroke=wire; }
					ha:line.8 { x1=320000; y1=472000; x2=320000; y2=472000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1139 {
				li:conn {
					/2/1138/1
					/2/724/12/1
				}
			}
			ha:connection.1140 {
				li:conn {
					/2/1138/3
					/2/724/11/1
				}
			}
			ha:connection.1141 {
				li:conn {
					/2/724/13/1
					/2/1138/7
				}
			}
			ha:connection.1143 {
				li:conn {
					/2/1138/7
					/2/1082/1/1
				}
			}
			ha:connection.1144 {
				li:conn {
					/2/1129/7
					/2/1081/1/1
				}
			}
			ha:group.1145 {
				uuid=jsnwx40oi7YLGck+mCkAAAK3; src_uuid=jsnwx40oi7YLGck+mCkAAAKo;
				x=-148000; y=-260000;
				li:objects {
					ha:line.1 { x1=296000; y1=448000; x2=308000; y2=448000; stroke=wire; }
					ha:text.2 { x1=296000; y1=448000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CS_GNL
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1146 {
				li:conn {
					/2/1145/1
					/2/580/10/1
				}
			}
			ha:group.1147 {
				uuid=jsnwx40oi7YLGck+mCkAAAK4; src_uuid=jsnwx40oi7YLGck+mCkAAAKl;
				x=-148000; y=-260000;
				li:objects {
					ha:line.1 { x1=296000; y1=460000; x2=308000; y2=460000; stroke=wire; }
					ha:text.2 { x1=296000; y1=460000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1148 {
				li:conn {
					/2/1147/1
					/2/580/7/1
				}
			}
			ha:group.1149 {
				uuid=jsnwx40oi7YLGck+mCkAAAK5; src_uuid=jsnwx40oi7YLGck+mCkAAAKm;
				x=-148000; y=-260000;
				li:objects {
					ha:line.1 { x1=296000; y1=456000; x2=308000; y2=456000; stroke=wire; }
					ha:text.2 { x1=296000; y1=456000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1150 {
				li:conn {
					/2/1149/1
					/2/580/8/1
				}
			}
			ha:group.1151 {
				uuid=jsnwx40oi7YLGck+mCkAAAK6; src_uuid=jsnwx40oi7YLGck+mCkAAAKn;
				x=-148000; y=-260000;
				li:objects {
					ha:line.1 { x1=296000; y1=452000; x2=308000; y2=452000; stroke=wire; }
					ha:text.2 { x1=296000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1152 {
				li:conn {
					/2/1151/1
					/2/580/9/1
				}
			}
			ha:group.1153 {
				uuid=jsnwx40oi7YLGck+mCkAAAK7; src_uuid=jsnwx40oi7YLGck+mCkAAAKo;
				x=-148000; y=-328000;
				li:objects {
					ha:line.1 { x1=296000; y1=448000; x2=308000; y2=448000; stroke=wire; }
					ha:text.2 { x1=296000; y1=448000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CS_GNR
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1154 {
				li:conn {
					/2/1153/1
					/2/348/10/1
				}
			}
			ha:group.1155 {
				uuid=jsnwx40oi7YLGck+mCkAAAK8; src_uuid=jsnwx40oi7YLGck+mCkAAAKl;
				x=-148000; y=-328000;
				li:objects {
					ha:line.1 { x1=296000; y1=460000; x2=308000; y2=460000; stroke=wire; }
					ha:text.2 { x1=296000; y1=460000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1156 {
				li:conn {
					/2/1155/1
					/2/348/7/1
				}
			}
			ha:group.1157 {
				uuid=jsnwx40oi7YLGck+mCkAAAK9; src_uuid=jsnwx40oi7YLGck+mCkAAAKm;
				x=-148000; y=-328000;
				li:objects {
					ha:line.1 { x1=296000; y1=456000; x2=308000; y2=456000; stroke=wire; }
					ha:text.2 { x1=296000; y1=456000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SDI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1158 {
				li:conn {
					/2/1157/1
					/2/348/8/1
				}
			}
			ha:group.1159 {
				uuid=jsnwx40oi7YLGck+mCkAAAK+; src_uuid=jsnwx40oi7YLGck+mCkAAAKn;
				x=-148000; y=-328000;
				li:objects {
					ha:line.1 { x1=296000; y1=452000; x2=308000; y2=452000; stroke=wire; }
					ha:text.2 { x1=296000; y1=452000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.1160 {
				li:conn {
					/2/1159/1
					/2/348/9/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=Matt Jenkins
			page=1
			print_page=A/4
			title=StereoInput
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 2
     grid = 4.0960 mm
    }
   }
  }
}
