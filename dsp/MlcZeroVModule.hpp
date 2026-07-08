/* ------------------------------------------------------------
name: "mlc_zero_v"
Code generated with Faust 2.85.1 (https://faust.grame.fr)
Compilation options: -lang cpp -i -fpga-mem-th 4 -ct 1 -cn mlczerov -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0 -vec -lv 0 -vs 32
------------------------------------------------------------ */

#ifndef  __mlczerov_H__
#define  __mlczerov_H__

#ifndef FAUSTFLOAT
#define FAUSTFLOAT float
#endif 

/* link with : "" */
#include <algorithm>
#include <cmath>
#include <cstdint>
#include <math.h>

#ifndef FAUSTCLASS 
#define FAUSTCLASS mlczerov
#endif

#ifdef __APPLE__ 
#define exp10f __exp10f
#define exp10 __exp10
#endif

#if defined(_WIN32)
#define RESTRICT __restrict
#else
#define RESTRICT __restrict__
#endif

static float mlczerov_faustpower2_f(float value) {
	return value * value;
}

class mlczerov : public dsp {
	
 private:
	
	int fSampleRate;
	float fConst0;
	float fConst1;
	float fConst2;
	FAUSTFLOAT fHslider0;
	float fRec0_perm[4];
	FAUSTFLOAT fHslider1;
	float fRec1_perm[4];
	float fRec52_perm[4];
	FAUSTFLOAT fHslider2;
	float fRec53_perm[4];
	float fRec51_perm[4];
	FAUSTFLOAT fHslider3;
	float fRec54_perm[4];
	float fConst3;
	float fConst4;
	float fConst5;
	float fConst6;
	float fConst7;
	float fRec46_perm[4];
	float fConst8;
	float fRec47_perm[4];
	float fConst9;
	float fConst10;
	float fConst11;
	FAUSTFLOAT fEntry0;
	FAUSTFLOAT fEntry1;
	float fConst12;
	float fConst13;
	float fConst14;
	float fRec42_perm[4];
	float fConst15;
	float fRec43_perm[4];
	float fConst16;
	FAUSTFLOAT fHslider4;
	float fRec55_perm[4];
	FAUSTFLOAT fHslider5;
	float fRec56_perm[4];
	FAUSTFLOAT fHslider6;
	float fRec57_perm[4];
	float fConst17;
	float fConst18;
	FAUSTFLOAT fEntry2;
	FAUSTFLOAT fEntry3;
	FAUSTFLOAT fEntry4;
	FAUSTFLOAT fEntry5;
	FAUSTFLOAT fEntry6;
	FAUSTFLOAT fEntry7;
	float fConst19;
	float fConst20;
	float fConst21;
	float fRec58_perm[4];
	float fConst22;
	float fRec59_perm[4];
	float fConst23;
	FAUSTFLOAT fEntry8;
	float fRec39_perm[4];
	float fRec40_perm[4];
	float fConst24;
	float fConst25;
	float fConst26;
	float fConst27;
	float fConst28;
	float fRec36_perm[4];
	float fConst29;
	float fRec37_perm[4];
	FAUSTFLOAT fEntry9;
	float fRec33_perm[4];
	float fRec34_perm[4];
	float fRec30_perm[4];
	float fRec31_perm[4];
	float fConst30;
	float fConst31;
	FAUSTFLOAT fEntry10;
	float fConst32;
	float fConst33;
	float fConst34;
	float fRec25_perm[4];
	float fConst35;
	float fRec26_perm[4];
	float fConst36;
	FAUSTFLOAT fHslider7;
	float fRec61_perm[4];
	float fConst37;
	float fConst38;
	float fConst39;
	float fConst40;
	float fConst41;
	float fRec21_perm[4];
	float fConst42;
	float fRec22_perm[4];
	float fConst43;
	FAUSTFLOAT fHslider8;
	float fRec62_perm[4];
	float fConst44;
	float fConst45;
	float fConst46;
	float fConst47;
	float fConst48;
	float fRec16_perm[4];
	float fConst49;
	float fRec17_perm[4];
	float fConst50;
	FAUSTFLOAT fHslider9;
	float fRec63_perm[4];
	FAUSTFLOAT fHslider10;
	float fRec64_perm[4];
	float fConst51;
	float fConst52;
	float fConst53;
	float fConst54;
	float fConst55;
	float fRec65_perm[4];
	float fConst56;
	float fRec66_perm[4];
	float fConst57;
	float fConst58;
	FAUSTFLOAT fEntry11;
	float fConst59;
	float fConst60;
	float fConst61;
	float fRec12_perm[4];
	float fConst62;
	float fRec13_perm[4];
	float fConst63;
	float fRec104_perm[4];
	float fRec105_perm[4];
	float fRec100_perm[4];
	float fRec101_perm[4];
	float fRec109_perm[4];
	float fRec110_perm[4];
	float fRec97_perm[4];
	float fRec98_perm[4];
	float fRec94_perm[4];
	float fRec95_perm[4];
	float fRec91_perm[4];
	float fRec92_perm[4];
	float fRec88_perm[4];
	float fRec89_perm[4];
	float fRec83_perm[4];
	float fRec84_perm[4];
	float fRec79_perm[4];
	float fRec80_perm[4];
	float fRec74_perm[4];
	float fRec75_perm[4];
	float fRec112_perm[4];
	float fRec113_perm[4];
	float fRec70_perm[4];
	float fRec71_perm[4];
	float fRec69_perm[4];
	float fRec68_perm[4];
	float fConst64;
	float fConst65;
	FAUSTFLOAT fEntry12;
	float fConst66;
	float fConst67;
	float fConst68;
	float fRec7_perm[4];
	float fConst69;
	float fRec8_perm[4];
	float fConst70;
	FAUSTFLOAT fHslider11;
	float fRec115_perm[4];
	float fConst71;
	float fConst72;
	FAUSTFLOAT fEntry13;
	float fConst73;
	float fConst74;
	float fConst75;
	float fRec2_perm[4];
	float fConst76;
	float fRec3_perm[4];
	float fConst77;
	FAUSTFLOAT fHslider12;
	float fRec116_perm[4];
	float fConst78;
	float fConst79;
	float fConst80;
	float fConst81;
	float fConst82;
	float fRec117_perm[4];
	float fConst83;
	float fRec118_perm[4];
	float fRec170_perm[4];
	float fRec169_perm[4];
	float fRec164_perm[4];
	float fRec165_perm[4];
	float fRec160_perm[4];
	float fRec161_perm[4];
	float fRec171_perm[4];
	float fRec172_perm[4];
	float fRec157_perm[4];
	float fRec158_perm[4];
	float fRec154_perm[4];
	float fRec155_perm[4];
	float fRec151_perm[4];
	float fRec152_perm[4];
	float fRec148_perm[4];
	float fRec149_perm[4];
	float fRec143_perm[4];
	float fRec144_perm[4];
	float fRec139_perm[4];
	float fRec140_perm[4];
	float fRec134_perm[4];
	float fRec135_perm[4];
	float fRec174_perm[4];
	float fRec175_perm[4];
	float fRec130_perm[4];
	float fRec131_perm[4];
	float fRec213_perm[4];
	float fRec214_perm[4];
	float fRec209_perm[4];
	float fRec210_perm[4];
	float fRec218_perm[4];
	float fRec219_perm[4];
	float fRec206_perm[4];
	float fRec207_perm[4];
	float fRec203_perm[4];
	float fRec204_perm[4];
	float fRec200_perm[4];
	float fRec201_perm[4];
	float fRec197_perm[4];
	float fRec198_perm[4];
	float fRec192_perm[4];
	float fRec193_perm[4];
	float fRec188_perm[4];
	float fRec189_perm[4];
	float fRec183_perm[4];
	float fRec184_perm[4];
	float fRec221_perm[4];
	float fRec222_perm[4];
	float fRec179_perm[4];
	float fRec180_perm[4];
	float fRec178_perm[4];
	float fRec177_perm[4];
	float fRec125_perm[4];
	float fRec126_perm[4];
	float fRec120_perm[4];
	float fRec121_perm[4];
	float fRec224_perm[4];
	float fRec225_perm[4];
	
 public:
	mlczerov() {
	}
	
	mlczerov(const mlczerov&) = default;
	
	virtual ~mlczerov() = default;
	
	mlczerov& operator=(const mlczerov&) = default;
	
	void metadata(Meta* m) { 
		m->declare("basics.lib/name", "Faust Basic Element Library");
		m->declare("basics.lib/version", "1.22.0");
		m->declare("compile_options", "-lang cpp -i -fpga-mem-th 4 -ct 1 -cn mlczerov -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0 -vec -lv 0 -vs 32");
		m->declare("filename", "mlc_zero_v.dsp");
		m->declare("maths.lib/author", "GRAME");
		m->declare("maths.lib/copyright", "GRAME");
		m->declare("maths.lib/license", "LGPL with exception");
		m->declare("maths.lib/name", "Faust Math Library");
		m->declare("maths.lib/version", "2.9.0");
		m->declare("name", "mlc_zero_v");
		m->declare("platform.lib/name", "Generic Platform Library");
		m->declare("platform.lib/version", "1.3.0");
		m->declare("signals.lib/name", "Faust Signal Routing Library");
		m->declare("signals.lib/version", "1.6.0");
	}

	virtual int getNumInputs() {
		return 2;
	}
	virtual int getNumOutputs() {
		return 2;
	}
	
	static void classInit(int sample_rate) {
	}
	
	virtual void instanceConstants(int sample_rate) {
		fSampleRate = sample_rate;
		fConst0 = std::min<float>(1.92e+05f, std::max<float>(1.0f, static_cast<float>(fSampleRate)));
		fConst1 = 44.1f / fConst0;
		fConst2 = 1.0f - fConst1;
		fConst3 = std::tan(471.2389f / fConst0);
		fConst4 = fConst3 + 1.4285715f;
		fConst5 = fConst3 * fConst4 + 1.0f;
		fConst6 = fConst3 / fConst5;
		fConst7 = 2.0f * fConst6;
		fConst8 = 2.0f * fConst3;
		fConst9 = 1.0f / fConst5;
		fConst10 = std::tan(3141.5928f / fConst0);
		fConst11 = fConst10 + 1.25f;
		fConst12 = fConst10 * fConst11 + 1.0f;
		fConst13 = fConst10 / fConst12;
		fConst14 = 2.0f * fConst13;
		fConst15 = 2.0f * fConst10;
		fConst16 = 1.0f / fConst12;
		fConst17 = std::tan(251.32741f / fConst0);
		fConst18 = fConst17 + 1.4144272f;
		fConst19 = fConst17 * fConst18 + 1.0f;
		fConst20 = fConst17 / fConst19;
		fConst21 = 2.0f * fConst20;
		fConst22 = 2.0f * fConst17;
		fConst23 = 1.0f / fConst19;
		fConst24 = std::tan(37699.113f / fConst0);
		fConst25 = fConst24 + 1.4144272f;
		fConst26 = fConst24 * fConst25 + 1.0f;
		fConst27 = fConst24 / fConst26;
		fConst28 = 2.0f * fConst27;
		fConst29 = 2.0f * fConst24;
		fConst30 = std::tan(376.99112f / fConst0);
		fConst31 = fConst30 + 1.4144272f;
		fConst32 = fConst30 * fConst31 + 1.0f;
		fConst33 = fConst30 / fConst32;
		fConst34 = 2.0f * fConst33;
		fConst35 = 2.0f * fConst30;
		fConst36 = 1.0f / fConst32;
		fConst37 = std::tan(2387.6104f / fConst0);
		fConst38 = fConst37 + 1.1764706f;
		fConst39 = fConst37 * fConst38 + 1.0f;
		fConst40 = fConst37 / fConst39;
		fConst41 = 2.0f * fConst40;
		fConst42 = 2.0f * fConst37;
		fConst43 = 1.0f / fConst39;
		fConst44 = std::tan(10681.415f / fConst0);
		fConst45 = fConst44 + 1.4144272f;
		fConst46 = fConst44 * fConst45 + 1.0f;
		fConst47 = fConst44 / fConst46;
		fConst48 = 2.0f * fConst47;
		fConst49 = 2.0f * fConst44;
		fConst50 = 1.0f / fConst46;
		fConst51 = std::tan(31.415926f / fConst0);
		fConst52 = fConst51 + 1.4144272f;
		fConst53 = fConst51 * fConst52 + 1.0f;
		fConst54 = fConst51 / fConst53;
		fConst55 = 2.0f * fConst54;
		fConst56 = 2.0f * fConst51;
		fConst57 = std::tan(2984.513f / fConst0);
		fConst58 = fConst57 + 0.8333333f;
		fConst59 = fConst57 * fConst58 + 1.0f;
		fConst60 = fConst57 / fConst59;
		fConst61 = 2.0f * fConst60;
		fConst62 = 2.0f * fConst57;
		fConst63 = 1.0f / fConst59;
		fConst64 = std::tan(298.4513f / fConst0);
		fConst65 = fConst64 + 1.25f;
		fConst66 = fConst64 * fConst65 + 1.0f;
		fConst67 = fConst64 / fConst66;
		fConst68 = 2.0f * fConst67;
		fConst69 = 2.0f * fConst64;
		fConst70 = 1.0f / fConst66;
		fConst71 = std::tan(11309.733f / fConst0);
		fConst72 = fConst71 + 1.4285715f;
		fConst73 = fConst71 * fConst72 + 1.0f;
		fConst74 = fConst71 / fConst73;
		fConst75 = 2.0f * fConst74;
		fConst76 = 2.0f * fConst71;
		fConst77 = 1.0f / fConst73;
		fConst78 = std::tan(25132.742f / fConst0);
		fConst79 = fConst78 + 1.4144272f;
		fConst80 = fConst78 * fConst79 + 1.0f;
		fConst81 = fConst78 / fConst80;
		fConst82 = 2.0f * fConst81;
		fConst83 = 2.0f * fConst78;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(0.5011872f);
		fHslider1 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider2 = static_cast<FAUSTFLOAT>(-8e+01f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.25118864f);
		fEntry0 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry1 = static_cast<FAUSTFLOAT>(-3.0f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider5 = static_cast<FAUSTFLOAT>(0.2f);
		fHslider6 = static_cast<FAUSTFLOAT>(0.7f);
		fEntry2 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry3 = static_cast<FAUSTFLOAT>(1.0f);
		fEntry4 = static_cast<FAUSTFLOAT>(3.0f);
		fEntry5 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry6 = static_cast<FAUSTFLOAT>(1.0f);
		fEntry7 = static_cast<FAUSTFLOAT>(0.5f);
		fEntry8 = static_cast<FAUSTFLOAT>(1.0f);
		fEntry9 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry10 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider7 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider8 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider9 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider10 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry11 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry12 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider11 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry13 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider12 = static_cast<FAUSTFLOAT>(0.0f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 4; l0 = l0 + 1) {
			fRec0_perm[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 4; l1 = l1 + 1) {
			fRec1_perm[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 4; l2 = l2 + 1) {
			fRec52_perm[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 4; l3 = l3 + 1) {
			fRec53_perm[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 4; l4 = l4 + 1) {
			fRec51_perm[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 4; l5 = l5 + 1) {
			fRec54_perm[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 4; l6 = l6 + 1) {
			fRec46_perm[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 4; l7 = l7 + 1) {
			fRec47_perm[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 4; l8 = l8 + 1) {
			fRec42_perm[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 4; l9 = l9 + 1) {
			fRec43_perm[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 4; l10 = l10 + 1) {
			fRec55_perm[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 4; l11 = l11 + 1) {
			fRec56_perm[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 4; l12 = l12 + 1) {
			fRec57_perm[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 4; l13 = l13 + 1) {
			fRec58_perm[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 4; l14 = l14 + 1) {
			fRec59_perm[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 4; l15 = l15 + 1) {
			fRec39_perm[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 4; l16 = l16 + 1) {
			fRec40_perm[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 4; l17 = l17 + 1) {
			fRec36_perm[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 4; l18 = l18 + 1) {
			fRec37_perm[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 4; l19 = l19 + 1) {
			fRec33_perm[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 4; l20 = l20 + 1) {
			fRec34_perm[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 4; l21 = l21 + 1) {
			fRec30_perm[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 4; l22 = l22 + 1) {
			fRec31_perm[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 4; l23 = l23 + 1) {
			fRec25_perm[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 4; l24 = l24 + 1) {
			fRec26_perm[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 4; l25 = l25 + 1) {
			fRec61_perm[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 4; l26 = l26 + 1) {
			fRec21_perm[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 4; l27 = l27 + 1) {
			fRec22_perm[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 4; l28 = l28 + 1) {
			fRec62_perm[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 4; l29 = l29 + 1) {
			fRec16_perm[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 4; l30 = l30 + 1) {
			fRec17_perm[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 4; l31 = l31 + 1) {
			fRec63_perm[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 4; l32 = l32 + 1) {
			fRec64_perm[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 4; l33 = l33 + 1) {
			fRec65_perm[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 4; l34 = l34 + 1) {
			fRec66_perm[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 4; l35 = l35 + 1) {
			fRec12_perm[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 4; l36 = l36 + 1) {
			fRec13_perm[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 4; l37 = l37 + 1) {
			fRec104_perm[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 4; l38 = l38 + 1) {
			fRec105_perm[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 4; l39 = l39 + 1) {
			fRec100_perm[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 4; l40 = l40 + 1) {
			fRec101_perm[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 4; l41 = l41 + 1) {
			fRec109_perm[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 4; l42 = l42 + 1) {
			fRec110_perm[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 4; l43 = l43 + 1) {
			fRec97_perm[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 4; l44 = l44 + 1) {
			fRec98_perm[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 4; l45 = l45 + 1) {
			fRec94_perm[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 4; l46 = l46 + 1) {
			fRec95_perm[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 4; l47 = l47 + 1) {
			fRec91_perm[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 4; l48 = l48 + 1) {
			fRec92_perm[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 4; l49 = l49 + 1) {
			fRec88_perm[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 4; l50 = l50 + 1) {
			fRec89_perm[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 4; l51 = l51 + 1) {
			fRec83_perm[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 4; l52 = l52 + 1) {
			fRec84_perm[l52] = 0.0f;
		}
		for (int l53 = 0; l53 < 4; l53 = l53 + 1) {
			fRec79_perm[l53] = 0.0f;
		}
		for (int l54 = 0; l54 < 4; l54 = l54 + 1) {
			fRec80_perm[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 4; l55 = l55 + 1) {
			fRec74_perm[l55] = 0.0f;
		}
		for (int l56 = 0; l56 < 4; l56 = l56 + 1) {
			fRec75_perm[l56] = 0.0f;
		}
		for (int l57 = 0; l57 < 4; l57 = l57 + 1) {
			fRec112_perm[l57] = 0.0f;
		}
		for (int l58 = 0; l58 < 4; l58 = l58 + 1) {
			fRec113_perm[l58] = 0.0f;
		}
		for (int l59 = 0; l59 < 4; l59 = l59 + 1) {
			fRec70_perm[l59] = 0.0f;
		}
		for (int l60 = 0; l60 < 4; l60 = l60 + 1) {
			fRec71_perm[l60] = 0.0f;
		}
		for (int l61 = 0; l61 < 4; l61 = l61 + 1) {
			fRec69_perm[l61] = 0.0f;
		}
		for (int l62 = 0; l62 < 4; l62 = l62 + 1) {
			fRec68_perm[l62] = 0.0f;
		}
		for (int l63 = 0; l63 < 4; l63 = l63 + 1) {
			fRec7_perm[l63] = 0.0f;
		}
		for (int l64 = 0; l64 < 4; l64 = l64 + 1) {
			fRec8_perm[l64] = 0.0f;
		}
		for (int l65 = 0; l65 < 4; l65 = l65 + 1) {
			fRec115_perm[l65] = 0.0f;
		}
		for (int l66 = 0; l66 < 4; l66 = l66 + 1) {
			fRec2_perm[l66] = 0.0f;
		}
		for (int l67 = 0; l67 < 4; l67 = l67 + 1) {
			fRec3_perm[l67] = 0.0f;
		}
		for (int l68 = 0; l68 < 4; l68 = l68 + 1) {
			fRec116_perm[l68] = 0.0f;
		}
		for (int l69 = 0; l69 < 4; l69 = l69 + 1) {
			fRec117_perm[l69] = 0.0f;
		}
		for (int l70 = 0; l70 < 4; l70 = l70 + 1) {
			fRec118_perm[l70] = 0.0f;
		}
		for (int l71 = 0; l71 < 4; l71 = l71 + 1) {
			fRec170_perm[l71] = 0.0f;
		}
		for (int l72 = 0; l72 < 4; l72 = l72 + 1) {
			fRec169_perm[l72] = 0.0f;
		}
		for (int l73 = 0; l73 < 4; l73 = l73 + 1) {
			fRec164_perm[l73] = 0.0f;
		}
		for (int l74 = 0; l74 < 4; l74 = l74 + 1) {
			fRec165_perm[l74] = 0.0f;
		}
		for (int l75 = 0; l75 < 4; l75 = l75 + 1) {
			fRec160_perm[l75] = 0.0f;
		}
		for (int l76 = 0; l76 < 4; l76 = l76 + 1) {
			fRec161_perm[l76] = 0.0f;
		}
		for (int l77 = 0; l77 < 4; l77 = l77 + 1) {
			fRec171_perm[l77] = 0.0f;
		}
		for (int l78 = 0; l78 < 4; l78 = l78 + 1) {
			fRec172_perm[l78] = 0.0f;
		}
		for (int l79 = 0; l79 < 4; l79 = l79 + 1) {
			fRec157_perm[l79] = 0.0f;
		}
		for (int l80 = 0; l80 < 4; l80 = l80 + 1) {
			fRec158_perm[l80] = 0.0f;
		}
		for (int l81 = 0; l81 < 4; l81 = l81 + 1) {
			fRec154_perm[l81] = 0.0f;
		}
		for (int l82 = 0; l82 < 4; l82 = l82 + 1) {
			fRec155_perm[l82] = 0.0f;
		}
		for (int l83 = 0; l83 < 4; l83 = l83 + 1) {
			fRec151_perm[l83] = 0.0f;
		}
		for (int l84 = 0; l84 < 4; l84 = l84 + 1) {
			fRec152_perm[l84] = 0.0f;
		}
		for (int l85 = 0; l85 < 4; l85 = l85 + 1) {
			fRec148_perm[l85] = 0.0f;
		}
		for (int l86 = 0; l86 < 4; l86 = l86 + 1) {
			fRec149_perm[l86] = 0.0f;
		}
		for (int l87 = 0; l87 < 4; l87 = l87 + 1) {
			fRec143_perm[l87] = 0.0f;
		}
		for (int l88 = 0; l88 < 4; l88 = l88 + 1) {
			fRec144_perm[l88] = 0.0f;
		}
		for (int l89 = 0; l89 < 4; l89 = l89 + 1) {
			fRec139_perm[l89] = 0.0f;
		}
		for (int l90 = 0; l90 < 4; l90 = l90 + 1) {
			fRec140_perm[l90] = 0.0f;
		}
		for (int l91 = 0; l91 < 4; l91 = l91 + 1) {
			fRec134_perm[l91] = 0.0f;
		}
		for (int l92 = 0; l92 < 4; l92 = l92 + 1) {
			fRec135_perm[l92] = 0.0f;
		}
		for (int l93 = 0; l93 < 4; l93 = l93 + 1) {
			fRec174_perm[l93] = 0.0f;
		}
		for (int l94 = 0; l94 < 4; l94 = l94 + 1) {
			fRec175_perm[l94] = 0.0f;
		}
		for (int l95 = 0; l95 < 4; l95 = l95 + 1) {
			fRec130_perm[l95] = 0.0f;
		}
		for (int l96 = 0; l96 < 4; l96 = l96 + 1) {
			fRec131_perm[l96] = 0.0f;
		}
		for (int l97 = 0; l97 < 4; l97 = l97 + 1) {
			fRec213_perm[l97] = 0.0f;
		}
		for (int l98 = 0; l98 < 4; l98 = l98 + 1) {
			fRec214_perm[l98] = 0.0f;
		}
		for (int l99 = 0; l99 < 4; l99 = l99 + 1) {
			fRec209_perm[l99] = 0.0f;
		}
		for (int l100 = 0; l100 < 4; l100 = l100 + 1) {
			fRec210_perm[l100] = 0.0f;
		}
		for (int l101 = 0; l101 < 4; l101 = l101 + 1) {
			fRec218_perm[l101] = 0.0f;
		}
		for (int l102 = 0; l102 < 4; l102 = l102 + 1) {
			fRec219_perm[l102] = 0.0f;
		}
		for (int l103 = 0; l103 < 4; l103 = l103 + 1) {
			fRec206_perm[l103] = 0.0f;
		}
		for (int l104 = 0; l104 < 4; l104 = l104 + 1) {
			fRec207_perm[l104] = 0.0f;
		}
		for (int l105 = 0; l105 < 4; l105 = l105 + 1) {
			fRec203_perm[l105] = 0.0f;
		}
		for (int l106 = 0; l106 < 4; l106 = l106 + 1) {
			fRec204_perm[l106] = 0.0f;
		}
		for (int l107 = 0; l107 < 4; l107 = l107 + 1) {
			fRec200_perm[l107] = 0.0f;
		}
		for (int l108 = 0; l108 < 4; l108 = l108 + 1) {
			fRec201_perm[l108] = 0.0f;
		}
		for (int l109 = 0; l109 < 4; l109 = l109 + 1) {
			fRec197_perm[l109] = 0.0f;
		}
		for (int l110 = 0; l110 < 4; l110 = l110 + 1) {
			fRec198_perm[l110] = 0.0f;
		}
		for (int l111 = 0; l111 < 4; l111 = l111 + 1) {
			fRec192_perm[l111] = 0.0f;
		}
		for (int l112 = 0; l112 < 4; l112 = l112 + 1) {
			fRec193_perm[l112] = 0.0f;
		}
		for (int l113 = 0; l113 < 4; l113 = l113 + 1) {
			fRec188_perm[l113] = 0.0f;
		}
		for (int l114 = 0; l114 < 4; l114 = l114 + 1) {
			fRec189_perm[l114] = 0.0f;
		}
		for (int l115 = 0; l115 < 4; l115 = l115 + 1) {
			fRec183_perm[l115] = 0.0f;
		}
		for (int l116 = 0; l116 < 4; l116 = l116 + 1) {
			fRec184_perm[l116] = 0.0f;
		}
		for (int l117 = 0; l117 < 4; l117 = l117 + 1) {
			fRec221_perm[l117] = 0.0f;
		}
		for (int l118 = 0; l118 < 4; l118 = l118 + 1) {
			fRec222_perm[l118] = 0.0f;
		}
		for (int l119 = 0; l119 < 4; l119 = l119 + 1) {
			fRec179_perm[l119] = 0.0f;
		}
		for (int l120 = 0; l120 < 4; l120 = l120 + 1) {
			fRec180_perm[l120] = 0.0f;
		}
		for (int l121 = 0; l121 < 4; l121 = l121 + 1) {
			fRec178_perm[l121] = 0.0f;
		}
		for (int l122 = 0; l122 < 4; l122 = l122 + 1) {
			fRec177_perm[l122] = 0.0f;
		}
		for (int l123 = 0; l123 < 4; l123 = l123 + 1) {
			fRec125_perm[l123] = 0.0f;
		}
		for (int l124 = 0; l124 < 4; l124 = l124 + 1) {
			fRec126_perm[l124] = 0.0f;
		}
		for (int l125 = 0; l125 < 4; l125 = l125 + 1) {
			fRec120_perm[l125] = 0.0f;
		}
		for (int l126 = 0; l126 < 4; l126 = l126 + 1) {
			fRec121_perm[l126] = 0.0f;
		}
		for (int l127 = 0; l127 < 4; l127 = l127 + 1) {
			fRec224_perm[l127] = 0.0f;
		}
		for (int l128 = 0; l128 < 4; l128 = l128 + 1) {
			fRec225_perm[l128] = 0.0f;
		}
	}
	
	virtual void init(int sample_rate) {
		classInit(sample_rate);
		instanceInit(sample_rate);
	}
	
	virtual void instanceInit(int sample_rate) {
		instanceConstants(sample_rate);
		instanceResetUserInterface();
		instanceClear();
	}
	
	virtual mlczerov* clone() {
		return new mlczerov(*this);
	}
	
	virtual int getSampleRate() {
		return fSampleRate;
	}
	
	virtual void buildUserInterface(UI* ui_interface) {
		ui_interface->openVerticalBox("mlc_zero_v");
		ui_interface->addNumEntry("Asymmetry", &fEntry7, FAUSTFLOAT(0.5f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->addNumEntry("Asymmetry Enable", &fEntry3, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider7, "unit", "dB");
		ui_interface->addHorizontalSlider("Bass", &fHslider7, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Bright", &fEntry6, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Clean Blend", &fHslider1, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.25f), FAUSTFLOAT(0.01f));
		ui_interface->addNumEntry("Clip Type 1", &fEntry2, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(2.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Clip Type 2", &fEntry9, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(2.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Clip Type 3", &fEntry10, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(2.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider11, "unit", "dB");
		ui_interface->addHorizontalSlider("Depth", &fHslider11, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Feedback", &fEntry13, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Gain", &fHslider3, FAUSTFLOAT(0.25118864f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->addNumEntry("Gate Pos", &fEntry12, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider2, "unit", "dB");
		ui_interface->addHorizontalSlider("Gate", &fHslider2, FAUSTFLOAT(-8e+01f), FAUSTFLOAT(-8e+01f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("H2", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->addHorizontalSlider("H3", &fHslider6, FAUSTFLOAT(0.7f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->addHorizontalSlider("H4", &fHslider5, FAUSTFLOAT(0.2f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->addNumEntry("M45", &fEntry5, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Master", &fHslider0, FAUSTFLOAT(0.5011872f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->declare(&fHslider8, "unit", "dB");
		ui_interface->addHorizontalSlider("Middle", &fHslider8, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Pre-Shape", &fEntry0, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Pre-Shape Bite", &fEntry4, FAUSTFLOAT(3.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(6.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Pre-Shape Tight", &fEntry1, FAUSTFLOAT(-3.0f), FAUSTFLOAT(-6.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider12, "unit", "dB");
		ui_interface->addHorizontalSlider("Presence", &fHslider12, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("Sag", &fHslider10, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->addNumEntry("Tight", &fEntry8, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider9, "unit", "dB");
		ui_interface->addHorizontalSlider("Treble", &fHslider9, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("WARCLAW", &fEntry11, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** RESTRICT inputs, FAUSTFLOAT** RESTRICT outputs) {
		FAUSTFLOAT* input0_ptr = inputs[0];
		FAUSTFLOAT* input1_ptr = inputs[1];
		FAUSTFLOAT* output0_ptr = outputs[0];
		FAUSTFLOAT* output1_ptr = outputs[1];
		float fSlow0 = fConst1 * static_cast<float>(fHslider0);
		float fRec0_tmp[36];
		float* fRec0 = &fRec0_tmp[4];
		float fSlow1 = fConst1 * static_cast<float>(fHslider1);
		float fRec1_tmp[36];
		float* fRec1 = &fRec1_tmp[4];
		float fRec52_tmp[36];
		float* fRec52 = &fRec52_tmp[4];
		float fSlow2 = fConst1 * static_cast<float>(fHslider2);
		float fRec53_tmp[36];
		float* fRec53 = &fRec53_tmp[4];
		float fZec0[32];
		float fRec51_tmp[36];
		float* fRec51 = &fRec51_tmp[4];
		float fSlow3 = fConst1 * static_cast<float>(fHslider3);
		float fRec54_tmp[36];
		float* fRec54 = &fRec54_tmp[4];
		float fZec1[32];
		float fRec46_tmp[36];
		float* fRec46 = &fRec46_tmp[4];
		float fZec2[32];
		float fRec47_tmp[36];
		float* fRec47 = &fRec47_tmp[4];
		float fZec3[32];
		float fRec48[32];
		float fZec4[32];
		float fRec49[32];
		float fRec50[32];
		float fSlow4 = static_cast<float>(fEntry0);
		float fSlow5 = std::pow(1e+01f, 0.05f * static_cast<float>(fEntry1) * fSlow4);
		float fSlow6 = 1.4285715f * std::sqrt(fSlow5);
		float fZec5[32];
		float fRec42_tmp[36];
		float* fRec42 = &fRec42_tmp[4];
		float fZec6[32];
		float fRec43_tmp[36];
		float* fRec43 = &fRec43_tmp[4];
		float fRec44[32];
		float fZec7[32];
		float fZec8[32];
		float fRec45[32];
		float fSlow7 = fConst1 * static_cast<float>(fHslider4);
		float fRec55_tmp[36];
		float* fRec55 = &fRec55_tmp[4];
		float fSlow8 = fConst1 * static_cast<float>(fHslider5);
		float fRec56_tmp[36];
		float* fRec56 = &fRec56_tmp[4];
		float fSlow9 = fConst1 * static_cast<float>(fHslider6);
		float fRec57_tmp[36];
		float* fRec57 = &fRec57_tmp[4];
		int iSlow10 = static_cast<int>(static_cast<float>(fEntry2));
		int iSlow11 = iSlow10 >= 2;
		int iSlow12 = iSlow10 >= 1;
		int iSlow13 = static_cast<float>(fEntry3) > 0.5f;
		float fZec9[32];
		float fZec10[32];
		float fSlow14 = std::pow(1e+01f, 0.05f * fSlow4 * static_cast<float>(fEntry4));
		float fZec11[32];
		float fZec12[32];
		float fSlow15 = 1.0f - 0.35f * static_cast<float>(fEntry5);
		float fSlow16 = 1.2f * static_cast<float>(fEntry6) + 1.5f;
		float fSlow17 = fSlow16 * fSlow15;
		float fSlow18 = 0.22f * fSlow17;
		float fZec13[32];
		float fSlow19 = 0.5f * static_cast<float>(fEntry7);
		float fSlow20 = tanhf(fSlow19);
		float fZec14[32];
		float fZec15[32];
		float fZec16[32];
		float fSlow21 = mlczerov_faustpower2_f(fSlow16) * mlczerov_faustpower2_f(fSlow15);
		float fSlow22 = 0.01874048f * fSlow21;
		float fSlow23 = 0.042592f * fSlow21;
		float fZec17[32];
		float fZec18[32];
		float fSlow24 = 0.0968f * fSlow21;
		float fZec19[32];
		float fZec20[32];
		float fRec58_tmp[36];
		float* fRec58 = &fRec58_tmp[4];
		float fZec21[32];
		float fRec59_tmp[36];
		float* fRec59 = &fRec59_tmp[4];
		float fZec22[32];
		float fRec60[32];
		int iSlow25 = static_cast<float>(fEntry8) > 0.5f;
		float fZec23[32];
		float fRec39_tmp[36];
		float* fRec39 = &fRec39_tmp[4];
		float fZec24[32];
		float fRec40_tmp[36];
		float* fRec40 = &fRec40_tmp[4];
		float fZec25[32];
		float fRec41[32];
		float fZec26[32];
		float fRec36_tmp[36];
		float* fRec36 = &fRec36_tmp[4];
		float fZec27[32];
		float fRec37_tmp[36];
		float* fRec37 = &fRec37_tmp[4];
		float fZec28[32];
		float fRec38[32];
		int iSlow26 = static_cast<int>(static_cast<float>(fEntry9));
		int iSlow27 = iSlow26 >= 2;
		int iSlow28 = iSlow26 >= 1;
		float fSlow29 = 0.34f * fSlow15;
		float fZec29[32];
		float fZec30[32];
		float fZec31[32];
		float fRec33_tmp[36];
		float* fRec33 = &fRec33_tmp[4];
		float fZec32[32];
		float fRec34_tmp[36];
		float* fRec34 = &fRec34_tmp[4];
		float fZec33[32];
		float fRec35[32];
		float fZec34[32];
		float fRec30_tmp[36];
		float* fRec30 = &fRec30_tmp[4];
		float fZec35[32];
		float fRec31_tmp[36];
		float* fRec31 = &fRec31_tmp[4];
		float fZec36[32];
		float fRec32[32];
		int iSlow30 = static_cast<int>(static_cast<float>(fEntry10));
		int iSlow31 = iSlow30 >= 2;
		int iSlow32 = iSlow30 >= 1;
		float fZec37[32];
		float fZec38[32];
		float fZec39[32];
		float fZec40[32];
		float fZec41[32];
		float fRec25_tmp[36];
		float* fRec25 = &fRec25_tmp[4];
		float fZec42[32];
		float fRec26_tmp[36];
		float* fRec26 = &fRec26_tmp[4];
		float fZec43[32];
		float fRec27[32];
		float fZec44[32];
		float fRec28[32];
		float fRec29[32];
		float fSlow33 = fConst1 * static_cast<float>(fHslider7);
		float fRec61_tmp[36];
		float* fRec61 = &fRec61_tmp[4];
		float fZec45[32];
		float fZec46[32];
		float fZec47[32];
		float fRec21_tmp[36];
		float* fRec21 = &fRec21_tmp[4];
		float fZec48[32];
		float fRec22_tmp[36];
		float* fRec22 = &fRec22_tmp[4];
		float fRec23[32];
		float fZec49[32];
		float fZec50[32];
		float fRec24[32];
		float fSlow34 = fConst1 * static_cast<float>(fHslider8);
		float fRec62_tmp[36];
		float* fRec62 = &fRec62_tmp[4];
		float fZec51[32];
		float fZec52[32];
		float fRec16_tmp[36];
		float* fRec16 = &fRec16_tmp[4];
		float fZec53[32];
		float fRec17_tmp[36];
		float* fRec17 = &fRec17_tmp[4];
		float fZec54[32];
		float fRec18[32];
		float fZec55[32];
		float fRec19[32];
		float fRec20[32];
		float fSlow35 = fConst1 * static_cast<float>(fHslider9);
		float fRec63_tmp[36];
		float* fRec63 = &fRec63_tmp[4];
		float fSlow36 = fConst1 * static_cast<float>(fHslider10);
		float fRec64_tmp[36];
		float* fRec64 = &fRec64_tmp[4];
		float fZec56[32];
		float fZec57[32];
		float fZec58[32];
		float fZec59[32];
		float fRec65_tmp[36];
		float* fRec65 = &fRec65_tmp[4];
		float fZec60[32];
		float fRec66_tmp[36];
		float* fRec66 = &fRec66_tmp[4];
		float fZec61[32];
		float fRec67[32];
		float fSlow37 = static_cast<float>(fEntry11);
		float fSlow38 = 1.9f * fSlow37 + 1.0f;
		float fZec62[32];
		float fRec12_tmp[36];
		float* fRec12 = &fRec12_tmp[4];
		float fZec63[32];
		float fRec13_tmp[36];
		float* fRec13 = &fRec13_tmp[4];
		float fRec14[32];
		float fZec64[32];
		float fZec65[32];
		float fRec15[32];
		float fZec66[32];
		float fRec104_tmp[36];
		float* fRec104 = &fRec104_tmp[4];
		float fZec67[32];
		float fRec105_tmp[36];
		float* fRec105 = &fRec105_tmp[4];
		float fZec68[32];
		float fRec106[32];
		float fZec69[32];
		float fRec107[32];
		float fRec108[32];
		float fZec70[32];
		float fRec100_tmp[36];
		float* fRec100 = &fRec100_tmp[4];
		float fZec71[32];
		float fRec101_tmp[36];
		float* fRec101 = &fRec101_tmp[4];
		float fRec102[32];
		float fZec72[32];
		float fZec73[32];
		float fRec103[32];
		float fZec74[32];
		float fZec75[32];
		float fZec76[32];
		float fZec77[32];
		float fZec78[32];
		float fZec79[32];
		float fZec80[32];
		float fZec81[32];
		float fRec109_tmp[36];
		float* fRec109 = &fRec109_tmp[4];
		float fZec82[32];
		float fRec110_tmp[36];
		float* fRec110 = &fRec110_tmp[4];
		float fZec83[32];
		float fRec111[32];
		float fZec84[32];
		float fRec97_tmp[36];
		float* fRec97 = &fRec97_tmp[4];
		float fZec85[32];
		float fRec98_tmp[36];
		float* fRec98 = &fRec98_tmp[4];
		float fZec86[32];
		float fRec99[32];
		float fZec87[32];
		float fRec94_tmp[36];
		float* fRec94 = &fRec94_tmp[4];
		float fZec88[32];
		float fRec95_tmp[36];
		float* fRec95 = &fRec95_tmp[4];
		float fZec89[32];
		float fRec96[32];
		float fZec90[32];
		float fZec91[32];
		float fZec92[32];
		float fRec91_tmp[36];
		float* fRec91 = &fRec91_tmp[4];
		float fZec93[32];
		float fRec92_tmp[36];
		float* fRec92 = &fRec92_tmp[4];
		float fZec94[32];
		float fRec93[32];
		float fZec95[32];
		float fRec88_tmp[36];
		float* fRec88 = &fRec88_tmp[4];
		float fZec96[32];
		float fRec89_tmp[36];
		float* fRec89 = &fRec89_tmp[4];
		float fZec97[32];
		float fRec90[32];
		float fZec98[32];
		float fZec99[32];
		float fZec100[32];
		float fZec101[32];
		float fRec83_tmp[36];
		float* fRec83 = &fRec83_tmp[4];
		float fZec102[32];
		float fRec84_tmp[36];
		float* fRec84 = &fRec84_tmp[4];
		float fZec103[32];
		float fRec85[32];
		float fZec104[32];
		float fRec86[32];
		float fRec87[32];
		float fZec105[32];
		float fRec79_tmp[36];
		float* fRec79 = &fRec79_tmp[4];
		float fZec106[32];
		float fRec80_tmp[36];
		float* fRec80 = &fRec80_tmp[4];
		float fRec81[32];
		float fZec107[32];
		float fZec108[32];
		float fRec82[32];
		float fZec109[32];
		float fRec74_tmp[36];
		float* fRec74 = &fRec74_tmp[4];
		float fZec110[32];
		float fRec75_tmp[36];
		float* fRec75 = &fRec75_tmp[4];
		float fZec111[32];
		float fRec76[32];
		float fZec112[32];
		float fRec77[32];
		float fRec78[32];
		float fZec113[32];
		float fZec114[32];
		float fRec112_tmp[36];
		float* fRec112 = &fRec112_tmp[4];
		float fZec115[32];
		float fRec113_tmp[36];
		float* fRec113 = &fRec113_tmp[4];
		float fZec116[32];
		float fRec114[32];
		float fZec117[32];
		float fRec70_tmp[36];
		float* fRec70 = &fRec70_tmp[4];
		float fZec118[32];
		float fRec71_tmp[36];
		float* fRec71 = &fRec71_tmp[4];
		float fRec72[32];
		float fZec119[32];
		float fZec120[32];
		float fRec73[32];
		float fSlow39 = std::pow(1e+01f, 0.2f * fSlow37);
		float fZec121[32];
		float fZec122[32];
		float fZec123[32];
		float fSlow40 = 1.0f - 0.22f * fSlow37;
		float fRec69_tmp[36];
		float* fRec69 = &fRec69_tmp[4];
		float fRec68_tmp[36];
		float* fRec68 = &fRec68_tmp[4];
		int iSlow41 = static_cast<int>(static_cast<float>(fEntry12));
		float fZec124[32];
		float fZec125[32];
		float fZec126[32];
		float fRec7_tmp[36];
		float* fRec7 = &fRec7_tmp[4];
		float fZec127[32];
		float fRec8_tmp[36];
		float* fRec8 = &fRec8_tmp[4];
		float fZec128[32];
		float fRec9[32];
		float fZec129[32];
		float fRec10[32];
		float fRec11[32];
		float fSlow42 = fConst1 * static_cast<float>(fHslider11);
		float fRec115_tmp[36];
		float* fRec115 = &fRec115_tmp[4];
		float fSlow43 = static_cast<float>(fEntry13);
		float fSlow44 = 0.05f * (1.25f - 0.35f * fSlow43);
		float fZec130[32];
		float fZec131[32];
		float fZec132[32];
		float fRec2_tmp[36];
		float* fRec2 = &fRec2_tmp[4];
		float fZec133[32];
		float fRec3_tmp[36];
		float* fRec3 = &fRec3_tmp[4];
		float fZec134[32];
		float fRec4[32];
		float fZec135[32];
		float fRec5[32];
		float fRec6[32];
		float fSlow45 = fConst1 * static_cast<float>(fHslider12);
		float fRec116_tmp[36];
		float* fRec116 = &fRec116_tmp[4];
		float fZec136[32];
		float fRec117_tmp[36];
		float* fRec117 = &fRec117_tmp[4];
		float fZec137[32];
		float fRec118_tmp[36];
		float* fRec118 = &fRec118_tmp[4];
		float fZec138[32];
		float fRec119[32];
		float fSlow46 = 0.05f * (0.25f * fSlow43 + 0.75f);
		float fZec139[32];
		float fZec140[32];
		float fZec141[32];
		float fRec170_tmp[36];
		float* fRec170 = &fRec170_tmp[4];
		float fRec169_tmp[36];
		float* fRec169 = &fRec169_tmp[4];
		float fZec142[32];
		float fZec143[32];
		float fRec164_tmp[36];
		float* fRec164 = &fRec164_tmp[4];
		float fZec144[32];
		float fRec165_tmp[36];
		float* fRec165 = &fRec165_tmp[4];
		float fZec145[32];
		float fRec166[32];
		float fZec146[32];
		float fRec167[32];
		float fRec168[32];
		float fZec147[32];
		float fRec160_tmp[36];
		float* fRec160 = &fRec160_tmp[4];
		float fZec148[32];
		float fRec161_tmp[36];
		float* fRec161 = &fRec161_tmp[4];
		float fRec162[32];
		float fZec149[32];
		float fZec150[32];
		float fRec163[32];
		float fZec151[32];
		float fZec152[32];
		float fZec153[32];
		float fZec154[32];
		float fZec155[32];
		float fZec156[32];
		float fRec171_tmp[36];
		float* fRec171 = &fRec171_tmp[4];
		float fZec157[32];
		float fRec172_tmp[36];
		float* fRec172 = &fRec172_tmp[4];
		float fZec158[32];
		float fRec173[32];
		float fZec159[32];
		float fRec157_tmp[36];
		float* fRec157 = &fRec157_tmp[4];
		float fZec160[32];
		float fRec158_tmp[36];
		float* fRec158 = &fRec158_tmp[4];
		float fZec161[32];
		float fRec159[32];
		float fZec162[32];
		float fRec154_tmp[36];
		float* fRec154 = &fRec154_tmp[4];
		float fZec163[32];
		float fRec155_tmp[36];
		float* fRec155 = &fRec155_tmp[4];
		float fZec164[32];
		float fRec156[32];
		float fZec165[32];
		float fZec166[32];
		float fZec167[32];
		float fRec151_tmp[36];
		float* fRec151 = &fRec151_tmp[4];
		float fZec168[32];
		float fRec152_tmp[36];
		float* fRec152 = &fRec152_tmp[4];
		float fZec169[32];
		float fRec153[32];
		float fZec170[32];
		float fRec148_tmp[36];
		float* fRec148 = &fRec148_tmp[4];
		float fZec171[32];
		float fRec149_tmp[36];
		float* fRec149 = &fRec149_tmp[4];
		float fZec172[32];
		float fRec150[32];
		float fZec173[32];
		float fZec174[32];
		float fZec175[32];
		float fZec176[32];
		float fRec143_tmp[36];
		float* fRec143 = &fRec143_tmp[4];
		float fZec177[32];
		float fRec144_tmp[36];
		float* fRec144 = &fRec144_tmp[4];
		float fZec178[32];
		float fRec145[32];
		float fZec179[32];
		float fRec146[32];
		float fRec147[32];
		float fZec180[32];
		float fRec139_tmp[36];
		float* fRec139 = &fRec139_tmp[4];
		float fZec181[32];
		float fRec140_tmp[36];
		float* fRec140 = &fRec140_tmp[4];
		float fRec141[32];
		float fZec182[32];
		float fZec183[32];
		float fRec142[32];
		float fZec184[32];
		float fRec134_tmp[36];
		float* fRec134 = &fRec134_tmp[4];
		float fZec185[32];
		float fRec135_tmp[36];
		float* fRec135 = &fRec135_tmp[4];
		float fZec186[32];
		float fRec136[32];
		float fZec187[32];
		float fRec137[32];
		float fRec138[32];
		float fZec188[32];
		float fZec189[32];
		float fRec174_tmp[36];
		float* fRec174 = &fRec174_tmp[4];
		float fZec190[32];
		float fRec175_tmp[36];
		float* fRec175 = &fRec175_tmp[4];
		float fZec191[32];
		float fRec176[32];
		float fZec192[32];
		float fRec130_tmp[36];
		float* fRec130 = &fRec130_tmp[4];
		float fZec193[32];
		float fRec131_tmp[36];
		float* fRec131 = &fRec131_tmp[4];
		float fRec132[32];
		float fZec194[32];
		float fZec195[32];
		float fRec133[32];
		float fZec196[32];
		float fRec213_tmp[36];
		float* fRec213 = &fRec213_tmp[4];
		float fZec197[32];
		float fRec214_tmp[36];
		float* fRec214 = &fRec214_tmp[4];
		float fZec198[32];
		float fRec215[32];
		float fZec199[32];
		float fRec216[32];
		float fRec217[32];
		float fZec200[32];
		float fRec209_tmp[36];
		float* fRec209 = &fRec209_tmp[4];
		float fZec201[32];
		float fRec210_tmp[36];
		float* fRec210 = &fRec210_tmp[4];
		float fRec211[32];
		float fZec202[32];
		float fZec203[32];
		float fRec212[32];
		float fZec204[32];
		float fZec205[32];
		float fZec206[32];
		float fZec207[32];
		float fZec208[32];
		float fZec209[32];
		float fRec218_tmp[36];
		float* fRec218 = &fRec218_tmp[4];
		float fZec210[32];
		float fRec219_tmp[36];
		float* fRec219 = &fRec219_tmp[4];
		float fZec211[32];
		float fRec220[32];
		float fZec212[32];
		float fRec206_tmp[36];
		float* fRec206 = &fRec206_tmp[4];
		float fZec213[32];
		float fRec207_tmp[36];
		float* fRec207 = &fRec207_tmp[4];
		float fZec214[32];
		float fRec208[32];
		float fZec215[32];
		float fRec203_tmp[36];
		float* fRec203 = &fRec203_tmp[4];
		float fZec216[32];
		float fRec204_tmp[36];
		float* fRec204 = &fRec204_tmp[4];
		float fZec217[32];
		float fRec205[32];
		float fZec218[32];
		float fZec219[32];
		float fZec220[32];
		float fRec200_tmp[36];
		float* fRec200 = &fRec200_tmp[4];
		float fZec221[32];
		float fRec201_tmp[36];
		float* fRec201 = &fRec201_tmp[4];
		float fZec222[32];
		float fRec202[32];
		float fZec223[32];
		float fRec197_tmp[36];
		float* fRec197 = &fRec197_tmp[4];
		float fZec224[32];
		float fRec198_tmp[36];
		float* fRec198 = &fRec198_tmp[4];
		float fZec225[32];
		float fRec199[32];
		float fZec226[32];
		float fZec227[32];
		float fZec228[32];
		float fZec229[32];
		float fRec192_tmp[36];
		float* fRec192 = &fRec192_tmp[4];
		float fZec230[32];
		float fRec193_tmp[36];
		float* fRec193 = &fRec193_tmp[4];
		float fZec231[32];
		float fRec194[32];
		float fZec232[32];
		float fRec195[32];
		float fRec196[32];
		float fZec233[32];
		float fRec188_tmp[36];
		float* fRec188 = &fRec188_tmp[4];
		float fZec234[32];
		float fRec189_tmp[36];
		float* fRec189 = &fRec189_tmp[4];
		float fRec190[32];
		float fZec235[32];
		float fZec236[32];
		float fRec191[32];
		float fZec237[32];
		float fRec183_tmp[36];
		float* fRec183 = &fRec183_tmp[4];
		float fZec238[32];
		float fRec184_tmp[36];
		float* fRec184 = &fRec184_tmp[4];
		float fZec239[32];
		float fRec185[32];
		float fZec240[32];
		float fRec186[32];
		float fRec187[32];
		float fZec241[32];
		float fZec242[32];
		float fRec221_tmp[36];
		float* fRec221 = &fRec221_tmp[4];
		float fZec243[32];
		float fRec222_tmp[36];
		float* fRec222 = &fRec222_tmp[4];
		float fZec244[32];
		float fRec223[32];
		float fZec245[32];
		float fRec179_tmp[36];
		float* fRec179 = &fRec179_tmp[4];
		float fZec246[32];
		float fRec180_tmp[36];
		float* fRec180 = &fRec180_tmp[4];
		float fRec181[32];
		float fZec247[32];
		float fZec248[32];
		float fRec182[32];
		float fZec249[32];
		float fZec250[32];
		float fZec251[32];
		float fRec178_tmp[36];
		float* fRec178 = &fRec178_tmp[4];
		float fRec177_tmp[36];
		float* fRec177 = &fRec177_tmp[4];
		float fZec252[32];
		float fZec253[32];
		float fZec254[32];
		float fRec125_tmp[36];
		float* fRec125 = &fRec125_tmp[4];
		float fZec255[32];
		float fRec126_tmp[36];
		float* fRec126 = &fRec126_tmp[4];
		float fZec256[32];
		float fRec127[32];
		float fZec257[32];
		float fRec128[32];
		float fRec129[32];
		float fZec258[32];
		float fRec120_tmp[36];
		float* fRec120 = &fRec120_tmp[4];
		float fZec259[32];
		float fRec121_tmp[36];
		float* fRec121 = &fRec121_tmp[4];
		float fZec260[32];
		float fRec122[32];
		float fZec261[32];
		float fRec123[32];
		float fRec124[32];
		float fZec262[32];
		float fRec224_tmp[36];
		float* fRec224 = &fRec224_tmp[4];
		float fZec263[32];
		float fRec225_tmp[36];
		float* fRec225 = &fRec225_tmp[4];
		float fZec264[32];
		float fRec226[32];
		int vindex = 0;
		/* Main loop */
		for (vindex = 0; vindex <= (count - 32); vindex = vindex + 32) {
			FAUSTFLOAT* input0 = &input0_ptr[vindex];
			FAUSTFLOAT* input1 = &input1_ptr[vindex];
			FAUSTFLOAT* output0 = &output0_ptr[vindex];
			FAUSTFLOAT* output1 = &output1_ptr[vindex];
			int vsize = 32;
			/* Recursive loop 0 */
			/* Pre code */
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec54_tmp[j10] = fRec54_perm[j10];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec54[i] = fSlow3 + fConst2 * fRec54[i - 1];
			}
			/* Post code */
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec54_perm[j11] = fRec54_tmp[vsize + j11];
			}
			/* Vectorizable loop 1 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec142[i] = static_cast<float>(input1[i]) * fRec54[i];
			}
			/* Recursive loop 2 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec53_tmp[j6] = fRec53_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec53[i] = fSlow2 + fConst2 * fRec53[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec53_perm[j7] = fRec53_tmp[vsize + j7];
			}
			/* Recursive loop 3 */
			/* Pre code */
			for (int j74 = 0; j74 < 4; j74 = j74 + 1) {
				fRec104_tmp[j74] = fRec104_perm[j74];
			}
			for (int j76 = 0; j76 < 4; j76 = j76 + 1) {
				fRec105_tmp[j76] = fRec105_perm[j76];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec66[i] = static_cast<float>(input0[i]) * fRec54[i] - (fConst4 * fRec104[i - 1] + fRec105[i - 1]);
				fRec104[i] = fRec104[i - 1] + fConst7 * fZec66[i];
				fZec67[i] = fRec104[i - 1] + fConst6 * fZec66[i];
				fRec105[i] = fRec105[i - 1] + fConst8 * fZec67[i];
				fZec68[i] = fConst3 * fZec67[i];
				fRec106[i] = fRec105[i - 1] + fZec68[i];
				fZec69[i] = fConst9 * fZec66[i];
				fRec107[i] = fZec69[i];
				fRec108[i] = fZec67[i];
			}
			/* Post code */
			for (int j75 = 0; j75 < 4; j75 = j75 + 1) {
				fRec104_perm[j75] = fRec104_tmp[vsize + j75];
			}
			for (int j77 = 0; j77 < 4; j77 = j77 + 1) {
				fRec105_perm[j77] = fRec105_tmp[vsize + j77];
			}
			/* Recursive loop 4 */
			/* Pre code */
			for (int j194 = 0; j194 < 4; j194 = j194 + 1) {
				fRec213_tmp[j194] = fRec213_perm[j194];
			}
			for (int j196 = 0; j196 < 4; j196 = j196 + 1) {
				fRec214_tmp[j196] = fRec214_perm[j196];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec196[i] = fZec142[i] - (fConst4 * fRec213[i - 1] + fRec214[i - 1]);
				fRec213[i] = fRec213[i - 1] + fConst7 * fZec196[i];
				fZec197[i] = fRec213[i - 1] + fConst6 * fZec196[i];
				fRec214[i] = fRec214[i - 1] + fConst8 * fZec197[i];
				fZec198[i] = fConst3 * fZec197[i];
				fRec215[i] = fRec214[i - 1] + fZec198[i];
				fZec199[i] = fConst9 * fZec196[i];
				fRec216[i] = fZec199[i];
				fRec217[i] = fZec197[i];
			}
			/* Post code */
			for (int j195 = 0; j195 < 4; j195 = j195 + 1) {
				fRec213_perm[j195] = fRec213_tmp[vsize + j195];
			}
			for (int j197 = 0; j197 < 4; j197 = j197 + 1) {
				fRec214_perm[j197] = fRec214_tmp[vsize + j197];
			}
			/* Recursive loop 5 */
			/* Pre code */
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec52_tmp[j4] = fRec52_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec52[i] = std::max<float>(0.995f * fRec52[i - 1], std::fabs(static_cast<float>(input0[i])));
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec52_perm[j5] = fRec52_tmp[vsize + j5];
			}
			/* Vectorizable loop 6 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::pow(1e+01f, 0.05f * fRec53[i]);
			}
			/* Vectorizable loop 7 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = 1.0f - 0.7f * fRec54[i];
			}
			/* Vectorizable loop 8 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = 72.0f * fRec54[i] + 8.0f;
			}
			/* Recursive loop 9 */
			/* Pre code */
			for (int j78 = 0; j78 < 4; j78 = j78 + 1) {
				fRec100_tmp[j78] = fRec100_perm[j78];
			}
			for (int j80 = 0; j80 < 4; j80 = j80 + 1) {
				fRec101_tmp[j80] = fRec101_perm[j80];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec70[i] = fRec107[i] + fSlow5 * fRec106[i] + fSlow6 * fRec108[i] - (fConst11 * fRec100[i - 1] + fRec101[i - 1]);
				fRec100[i] = fRec100[i - 1] + fConst14 * fZec70[i];
				fZec71[i] = fRec100[i - 1] + fConst13 * fZec70[i];
				fRec101[i] = fRec101[i - 1] + fConst15 * fZec71[i];
				fRec102[i] = fZec71[i];
				fZec72[i] = fConst16 * fZec70[i];
				fZec73[i] = fConst10 * fZec71[i];
				fRec103[i] = fZec73[i] + fRec101[i - 1] + fZec72[i];
			}
			/* Post code */
			for (int j79 = 0; j79 < 4; j79 = j79 + 1) {
				fRec100_perm[j79] = fRec100_tmp[vsize + j79];
			}
			for (int j81 = 0; j81 < 4; j81 = j81 + 1) {
				fRec101_perm[j81] = fRec101_tmp[vsize + j81];
			}
			/* Recursive loop 10 */
			/* Pre code */
			for (int j142 = 0; j142 < 4; j142 = j142 + 1) {
				fRec170_tmp[j142] = fRec170_perm[j142];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec170[i] = std::max<float>(0.995f * fRec170[i - 1], std::fabs(static_cast<float>(input1[i])));
			}
			/* Post code */
			for (int j143 = 0; j143 < 4; j143 = j143 + 1) {
				fRec170_perm[j143] = fRec170_tmp[vsize + j143];
			}
			/* Recursive loop 11 */
			/* Pre code */
			for (int j198 = 0; j198 < 4; j198 = j198 + 1) {
				fRec209_tmp[j198] = fRec209_perm[j198];
			}
			for (int j200 = 0; j200 < 4; j200 = j200 + 1) {
				fRec210_tmp[j200] = fRec210_perm[j200];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec200[i] = fRec216[i] + fSlow5 * fRec215[i] + fSlow6 * fRec217[i] - (fConst11 * fRec209[i - 1] + fRec210[i - 1]);
				fRec209[i] = fRec209[i - 1] + fConst14 * fZec200[i];
				fZec201[i] = fRec209[i - 1] + fConst13 * fZec200[i];
				fRec210[i] = fRec210[i - 1] + fConst15 * fZec201[i];
				fRec211[i] = fZec201[i];
				fZec202[i] = fConst16 * fZec200[i];
				fZec203[i] = fConst10 * fZec201[i];
				fRec212[i] = fZec203[i] + fRec210[i - 1] + fZec202[i];
			}
			/* Post code */
			for (int j199 = 0; j199 < 4; j199 = j199 + 1) {
				fRec209_perm[j199] = fRec209_tmp[vsize + j199];
			}
			for (int j201 = 0; j201 < 4; j201 = j201 + 1) {
				fRec210_perm[j201] = fRec210_tmp[vsize + j201];
			}
			/* Recursive loop 12 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec51_tmp[j8] = fRec51_perm[j8];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec51[i] = fConst1 * static_cast<float>(fRec52[i] > fZec0[i]) + fConst2 * fRec51[i - 1];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec51_perm[j9] = fRec51_tmp[vsize + j9];
			}
			/* Recursive loop 13 */
			/* Pre code */
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec55_tmp[j20] = fRec55_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec55[i] = fSlow7 + fConst2 * fRec55[i - 1];
			}
			/* Post code */
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec55_perm[j21] = fRec55_tmp[vsize + j21];
			}
			/* Recursive loop 14 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec56_tmp[j22] = fRec56_perm[j22];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec56[i] = fSlow8 + fConst2 * fRec56[i - 1];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec56_perm[j23] = fRec56_tmp[vsize + j23];
			}
			/* Recursive loop 15 */
			/* Pre code */
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec57_tmp[j24] = fRec57_perm[j24];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec57[i] = fSlow9 + fConst2 * fRec57[i - 1];
			}
			/* Post code */
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec57_perm[j25] = fRec57_tmp[vsize + j25];
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec14[i] = mlczerov_faustpower2_f(fZec9[i]);
			}
			/* Vectorizable loop 17 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec15[i] = mlczerov_faustpower2_f(fZec10[i]);
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec74[i] = fRec103[i] + fSlow14 * fRec102[i];
			}
			/* Vectorizable loop 19 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec75[i] = fZec10[i] * fZec9[i];
			}
			/* Recursive loop 20 */
			/* Pre code */
			for (int j144 = 0; j144 < 4; j144 = j144 + 1) {
				fRec169_tmp[j144] = fRec169_perm[j144];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec169[i] = fConst1 * static_cast<float>(fRec170[i] > fZec0[i]) + fConst2 * fRec169[i - 1];
			}
			/* Post code */
			for (int j145 = 0; j145 < 4; j145 = j145 + 1) {
				fRec169_perm[j145] = fRec169_tmp[vsize + j145];
			}
			/* Vectorizable loop 21 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec204[i] = fRec212[i] + fSlow14 * fRec211[i];
			}
			/* Recursive loop 22 */
			/* Pre code */
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec46_tmp[j12] = fRec46_perm[j12];
			}
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec47_tmp[j14] = fRec47_perm[j14];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec1[i] = static_cast<float>(input0[i]) * fRec51[i] * fRec54[i] - (fConst4 * fRec46[i - 1] + fRec47[i - 1]);
				fRec46[i] = fRec46[i - 1] + fConst7 * fZec1[i];
				fZec2[i] = fRec46[i - 1] + fConst6 * fZec1[i];
				fRec47[i] = fRec47[i - 1] + fConst8 * fZec2[i];
				fZec3[i] = fConst3 * fZec2[i];
				fRec48[i] = fRec47[i - 1] + fZec3[i];
				fZec4[i] = fConst9 * fZec1[i];
				fRec49[i] = fZec4[i];
				fRec50[i] = fZec2[i];
			}
			/* Post code */
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec46_perm[j13] = fRec46_tmp[vsize + j13];
			}
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec47_perm[j15] = fRec47_tmp[vsize + j15];
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec17[i] = 1.0f - (fRec56[i] + fRec55[i] + fRec57[i]);
			}
			/* Vectorizable loop 24 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec76[i] = fZec75[i] * fZec74[i];
			}
			/* Vectorizable loop 25 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec78[i] = fZec15[i] * fZec14[i];
			}
			/* Recursive loop 26 */
			/* Pre code */
			for (int j146 = 0; j146 < 4; j146 = j146 + 1) {
				fRec164_tmp[j146] = fRec164_perm[j146];
			}
			for (int j148 = 0; j148 < 4; j148 = j148 + 1) {
				fRec165_tmp[j148] = fRec165_perm[j148];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec143[i] = fZec142[i] * fRec169[i] - (fConst4 * fRec164[i - 1] + fRec165[i - 1]);
				fRec164[i] = fRec164[i - 1] + fConst7 * fZec143[i];
				fZec144[i] = fRec164[i - 1] + fConst6 * fZec143[i];
				fRec165[i] = fRec165[i - 1] + fConst8 * fZec144[i];
				fZec145[i] = fConst3 * fZec144[i];
				fRec166[i] = fRec165[i - 1] + fZec145[i];
				fZec146[i] = fConst9 * fZec143[i];
				fRec167[i] = fZec146[i];
				fRec168[i] = fZec144[i];
			}
			/* Post code */
			for (int j147 = 0; j147 < 4; j147 = j147 + 1) {
				fRec164_perm[j147] = fRec164_tmp[vsize + j147];
			}
			for (int j149 = 0; j149 < 4; j149 = j149 + 1) {
				fRec165_perm[j149] = fRec165_tmp[vsize + j149];
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec205[i] = fZec75[i] * fZec204[i];
			}
			/* Recursive loop 28 */
			/* Pre code */
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec42_tmp[j16] = fRec42_perm[j16];
			}
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec43_tmp[j18] = fRec43_perm[j18];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec5[i] = fRec49[i] + fSlow5 * fRec48[i] + fSlow6 * fRec50[i] - (fConst11 * fRec42[i - 1] + fRec43[i - 1]);
				fRec42[i] = fRec42[i - 1] + fConst14 * fZec5[i];
				fZec6[i] = fRec42[i - 1] + fConst13 * fZec5[i];
				fRec43[i] = fRec43[i - 1] + fConst15 * fZec6[i];
				fRec44[i] = fZec6[i];
				fZec7[i] = fConst16 * fZec5[i];
				fZec8[i] = fConst10 * fZec6[i];
				fRec45[i] = fZec8[i] + fRec43[i - 1] + fZec7[i];
			}
			/* Post code */
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec42_perm[j17] = fRec42_tmp[vsize + j17];
			}
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec43_perm[j19] = fRec43_tmp[vsize + j19];
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec18[i] = 0.22f * fZec17[i];
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec77[i] = fSlow18 * fZec76[i];
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec79[i] = fZec78[i] * mlczerov_faustpower2_f(fZec74[i]);
			}
			/* Recursive loop 32 */
			/* Pre code */
			for (int j150 = 0; j150 < 4; j150 = j150 + 1) {
				fRec160_tmp[j150] = fRec160_perm[j150];
			}
			for (int j152 = 0; j152 < 4; j152 = j152 + 1) {
				fRec161_tmp[j152] = fRec161_perm[j152];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec147[i] = fRec167[i] + fSlow5 * fRec166[i] + fSlow6 * fRec168[i] - (fConst11 * fRec160[i - 1] + fRec161[i - 1]);
				fRec160[i] = fRec160[i - 1] + fConst14 * fZec147[i];
				fZec148[i] = fRec160[i - 1] + fConst13 * fZec147[i];
				fRec161[i] = fRec161[i - 1] + fConst15 * fZec148[i];
				fRec162[i] = fZec148[i];
				fZec149[i] = fConst16 * fZec147[i];
				fZec150[i] = fConst10 * fZec148[i];
				fRec163[i] = fZec150[i] + fRec161[i - 1] + fZec149[i];
			}
			/* Post code */
			for (int j151 = 0; j151 < 4; j151 = j151 + 1) {
				fRec160_perm[j151] = fRec160_tmp[vsize + j151];
			}
			for (int j153 = 0; j153 < 4; j153 = j153 + 1) {
				fRec161_perm[j153] = fRec161_tmp[vsize + j153];
			}
			/* Vectorizable loop 33 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec206[i] = fSlow18 * fZec205[i];
			}
			/* Vectorizable loop 34 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec207[i] = fZec78[i] * mlczerov_faustpower2_f(fZec204[i]);
			}
			/* Vectorizable loop 35 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = fRec45[i] + fSlow14 * fRec44[i];
			}
			/* Vectorizable loop 36 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec80[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec79[i] + -1.0f) + fSlow17 * fZec76[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec79[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec79[i] * (fSlow22 * fZec79[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec77[i])))) * ((fZec77[i] > 0.0f) ? 1.0f : ((fZec77[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec77[i]) - fSlow20 : tanhf(fZec77[i]))));
			}
			/* Vectorizable loop 37 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec151[i] = fRec163[i] + fSlow14 * fRec162[i];
			}
			/* Vectorizable loop 38 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec208[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec207[i] + -1.0f) + fSlow17 * fZec205[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec207[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec207[i] * (fSlow22 * fZec207[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec206[i])))) * ((fZec206[i] > 0.0f) ? 1.0f : ((fZec206[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec206[i]) - fSlow20 : tanhf(fZec206[i]))));
			}
			/* Vectorizable loop 39 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = fZec11[i] * fZec10[i] * fZec9[i];
			}
			/* Recursive loop 40 */
			/* Pre code */
			for (int j82 = 0; j82 < 4; j82 = j82 + 1) {
				fRec109_tmp[j82] = fRec109_perm[j82];
			}
			for (int j84 = 0; j84 < 4; j84 = j84 + 1) {
				fRec110_tmp[j84] = fRec110_perm[j84];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec81[i] = fZec80[i] - (fConst18 * fRec109[i - 1] + fRec110[i - 1]);
				fRec109[i] = fRec109[i - 1] + fConst21 * fZec81[i];
				fZec82[i] = fRec109[i - 1] + fConst20 * fZec81[i];
				fRec110[i] = fRec110[i - 1] + fConst22 * fZec82[i];
				fZec83[i] = fConst23 * fZec81[i];
				fRec111[i] = fZec83[i];
			}
			/* Post code */
			for (int j83 = 0; j83 < 4; j83 = j83 + 1) {
				fRec109_perm[j83] = fRec109_tmp[vsize + j83];
			}
			for (int j85 = 0; j85 < 4; j85 = j85 + 1) {
				fRec110_perm[j85] = fRec110_tmp[vsize + j85];
			}
			/* Vectorizable loop 41 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec152[i] = fZec75[i] * fZec151[i];
			}
			/* Recursive loop 42 */
			/* Pre code */
			for (int j202 = 0; j202 < 4; j202 = j202 + 1) {
				fRec218_tmp[j202] = fRec218_perm[j202];
			}
			for (int j204 = 0; j204 < 4; j204 = j204 + 1) {
				fRec219_tmp[j204] = fRec219_perm[j204];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec209[i] = fZec208[i] - (fConst18 * fRec218[i - 1] + fRec219[i - 1]);
				fRec218[i] = fRec218[i - 1] + fConst21 * fZec209[i];
				fZec210[i] = fRec218[i - 1] + fConst20 * fZec209[i];
				fRec219[i] = fRec219[i - 1] + fConst22 * fZec210[i];
				fZec211[i] = fConst23 * fZec209[i];
				fRec220[i] = fZec211[i];
			}
			/* Post code */
			for (int j203 = 0; j203 < 4; j203 = j203 + 1) {
				fRec218_perm[j203] = fRec218_tmp[vsize + j203];
			}
			for (int j205 = 0; j205 < 4; j205 = j205 + 1) {
				fRec219_perm[j205] = fRec219_tmp[vsize + j205];
			}
			/* Vectorizable loop 43 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec13[i] = fSlow18 * fZec12[i];
			}
			/* Vectorizable loop 44 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec16[i] = mlczerov_faustpower2_f(fZec11[i]) * fZec15[i] * fZec14[i];
			}
			/* Recursive loop 45 */
			/* Pre code */
			for (int j86 = 0; j86 < 4; j86 = j86 + 1) {
				fRec97_tmp[j86] = fRec97_perm[j86];
			}
			for (int j88 = 0; j88 < 4; j88 = j88 + 1) {
				fRec98_tmp[j88] = fRec98_perm[j88];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec84[i] = ((iSlow25) ? fRec111[i] : fZec80[i]) - (fConst18 * fRec97[i - 1] + fRec98[i - 1]);
				fRec97[i] = fRec97[i - 1] + fConst21 * fZec84[i];
				fZec85[i] = fRec97[i - 1] + fConst20 * fZec84[i];
				fRec98[i] = fRec98[i - 1] + fConst22 * fZec85[i];
				fZec86[i] = fConst23 * fZec84[i];
				fRec99[i] = fZec86[i];
			}
			/* Post code */
			for (int j87 = 0; j87 < 4; j87 = j87 + 1) {
				fRec97_perm[j87] = fRec97_tmp[vsize + j87];
			}
			for (int j89 = 0; j89 < 4; j89 = j89 + 1) {
				fRec98_perm[j89] = fRec98_tmp[vsize + j89];
			}
			/* Vectorizable loop 46 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec153[i] = fSlow18 * fZec152[i];
			}
			/* Vectorizable loop 47 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec154[i] = fZec78[i] * mlczerov_faustpower2_f(fZec151[i]);
			}
			/* Recursive loop 48 */
			/* Pre code */
			for (int j206 = 0; j206 < 4; j206 = j206 + 1) {
				fRec206_tmp[j206] = fRec206_perm[j206];
			}
			for (int j208 = 0; j208 < 4; j208 = j208 + 1) {
				fRec207_tmp[j208] = fRec207_perm[j208];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec212[i] = ((iSlow25) ? fRec220[i] : fZec208[i]) - (fConst18 * fRec206[i - 1] + fRec207[i - 1]);
				fRec206[i] = fRec206[i - 1] + fConst21 * fZec212[i];
				fZec213[i] = fRec206[i - 1] + fConst20 * fZec212[i];
				fRec207[i] = fRec207[i - 1] + fConst22 * fZec213[i];
				fZec214[i] = fConst23 * fZec212[i];
				fRec208[i] = fZec214[i];
			}
			/* Post code */
			for (int j207 = 0; j207 < 4; j207 = j207 + 1) {
				fRec206_perm[j207] = fRec206_tmp[vsize + j207];
			}
			for (int j209 = 0; j209 < 4; j209 = j209 + 1) {
				fRec207_perm[j209] = fRec207_tmp[vsize + j209];
			}
			/* Vectorizable loop 49 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec19[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec16[i] + -1.0f) + fSlow17 * fZec12[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec16[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec16[i] * (fSlow22 * fZec16[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec13[i])))) * ((fZec13[i] > 0.0f) ? 1.0f : ((fZec13[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec13[i]) - fSlow20 : tanhf(fZec13[i]))));
			}
			/* Recursive loop 50 */
			/* Pre code */
			for (int j90 = 0; j90 < 4; j90 = j90 + 1) {
				fRec94_tmp[j90] = fRec94_perm[j90];
			}
			for (int j92 = 0; j92 < 4; j92 = j92 + 1) {
				fRec95_tmp[j92] = fRec95_perm[j92];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec87[i] = fRec99[i] - (fConst25 * fRec94[i - 1] + fRec95[i - 1]);
				fRec94[i] = fRec94[i - 1] + fConst28 * fZec87[i];
				fZec88[i] = fRec94[i - 1] + fConst27 * fZec87[i];
				fRec95[i] = fRec95[i - 1] + fConst29 * fZec88[i];
				fZec89[i] = fConst24 * fZec88[i];
				fRec96[i] = fRec95[i - 1] + fZec89[i];
			}
			/* Post code */
			for (int j91 = 0; j91 < 4; j91 = j91 + 1) {
				fRec94_perm[j91] = fRec94_tmp[vsize + j91];
			}
			for (int j93 = 0; j93 < 4; j93 = j93 + 1) {
				fRec95_perm[j93] = fRec95_tmp[vsize + j93];
			}
			/* Vectorizable loop 51 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec155[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec154[i] + -1.0f) + fSlow17 * fZec152[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec154[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec154[i] * (fSlow22 * fZec154[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec153[i])))) * ((fZec153[i] > 0.0f) ? 1.0f : ((fZec153[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec153[i]) - fSlow20 : tanhf(fZec153[i]))));
			}
			/* Recursive loop 52 */
			/* Pre code */
			for (int j210 = 0; j210 < 4; j210 = j210 + 1) {
				fRec203_tmp[j210] = fRec203_perm[j210];
			}
			for (int j212 = 0; j212 < 4; j212 = j212 + 1) {
				fRec204_tmp[j212] = fRec204_perm[j212];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec215[i] = fRec208[i] - (fConst25 * fRec203[i - 1] + fRec204[i - 1]);
				fRec203[i] = fRec203[i - 1] + fConst28 * fZec215[i];
				fZec216[i] = fRec203[i - 1] + fConst27 * fZec215[i];
				fRec204[i] = fRec204[i - 1] + fConst29 * fZec216[i];
				fZec217[i] = fConst24 * fZec216[i];
				fRec205[i] = fRec204[i - 1] + fZec217[i];
			}
			/* Post code */
			for (int j211 = 0; j211 < 4; j211 = j211 + 1) {
				fRec203_perm[j211] = fRec203_tmp[vsize + j211];
			}
			for (int j213 = 0; j213 < 4; j213 = j213 + 1) {
				fRec204_perm[j213] = fRec204_tmp[vsize + j213];
			}
			/* Recursive loop 53 */
			/* Pre code */
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec58_tmp[j26] = fRec58_perm[j26];
			}
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec59_tmp[j28] = fRec59_perm[j28];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec20[i] = fZec19[i] - (fConst18 * fRec58[i - 1] + fRec59[i - 1]);
				fRec58[i] = fRec58[i - 1] + fConst21 * fZec20[i];
				fZec21[i] = fRec58[i - 1] + fConst20 * fZec20[i];
				fRec59[i] = fRec59[i - 1] + fConst22 * fZec21[i];
				fZec22[i] = fConst23 * fZec20[i];
				fRec60[i] = fZec22[i];
			}
			/* Post code */
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec58_perm[j27] = fRec58_tmp[vsize + j27];
			}
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec59_perm[j29] = fRec59_tmp[vsize + j29];
			}
			/* Vectorizable loop 54 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec90[i] = fSlow29 * fRec96[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 55 */
			/* Pre code */
			for (int j154 = 0; j154 < 4; j154 = j154 + 1) {
				fRec171_tmp[j154] = fRec171_perm[j154];
			}
			for (int j156 = 0; j156 < 4; j156 = j156 + 1) {
				fRec172_tmp[j156] = fRec172_perm[j156];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec156[i] = fZec155[i] - (fConst18 * fRec171[i - 1] + fRec172[i - 1]);
				fRec171[i] = fRec171[i - 1] + fConst21 * fZec156[i];
				fZec157[i] = fRec171[i - 1] + fConst20 * fZec156[i];
				fRec172[i] = fRec172[i - 1] + fConst22 * fZec157[i];
				fZec158[i] = fConst23 * fZec156[i];
				fRec173[i] = fZec158[i];
			}
			/* Post code */
			for (int j155 = 0; j155 < 4; j155 = j155 + 1) {
				fRec171_perm[j155] = fRec171_tmp[vsize + j155];
			}
			for (int j157 = 0; j157 < 4; j157 = j157 + 1) {
				fRec172_perm[j157] = fRec172_tmp[vsize + j157];
			}
			/* Vectorizable loop 56 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec218[i] = fSlow29 * fRec205[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 57 */
			/* Pre code */
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec39_tmp[j30] = fRec39_perm[j30];
			}
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec40_tmp[j32] = fRec40_perm[j32];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec23[i] = ((iSlow25) ? fRec60[i] : fZec19[i]) - (fConst18 * fRec39[i - 1] + fRec40[i - 1]);
				fRec39[i] = fRec39[i - 1] + fConst21 * fZec23[i];
				fZec24[i] = fRec39[i - 1] + fConst20 * fZec23[i];
				fRec40[i] = fRec40[i - 1] + fConst22 * fZec24[i];
				fZec25[i] = fConst23 * fZec23[i];
				fRec41[i] = fZec25[i];
			}
			/* Post code */
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec39_perm[j31] = fRec39_tmp[vsize + j31];
			}
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec40_perm[j33] = fRec40_tmp[vsize + j33];
			}
			/* Vectorizable loop 58 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec91[i] = mlczerov_faustpower2_f(fZec90[i]);
			}
			/* Recursive loop 59 */
			/* Pre code */
			for (int j158 = 0; j158 < 4; j158 = j158 + 1) {
				fRec157_tmp[j158] = fRec157_perm[j158];
			}
			for (int j160 = 0; j160 < 4; j160 = j160 + 1) {
				fRec158_tmp[j160] = fRec158_perm[j160];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec159[i] = ((iSlow25) ? fRec173[i] : fZec155[i]) - (fConst18 * fRec157[i - 1] + fRec158[i - 1]);
				fRec157[i] = fRec157[i - 1] + fConst21 * fZec159[i];
				fZec160[i] = fRec157[i - 1] + fConst20 * fZec159[i];
				fRec158[i] = fRec158[i - 1] + fConst22 * fZec160[i];
				fZec161[i] = fConst23 * fZec159[i];
				fRec159[i] = fZec161[i];
			}
			/* Post code */
			for (int j159 = 0; j159 < 4; j159 = j159 + 1) {
				fRec157_perm[j159] = fRec157_tmp[vsize + j159];
			}
			for (int j161 = 0; j161 < 4; j161 = j161 + 1) {
				fRec158_perm[j161] = fRec158_tmp[vsize + j161];
			}
			/* Vectorizable loop 60 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec219[i] = mlczerov_faustpower2_f(fZec218[i]);
			}
			/* Recursive loop 61 */
			/* Pre code */
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec36_tmp[j34] = fRec36_perm[j34];
			}
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec37_tmp[j36] = fRec37_perm[j36];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec26[i] = fRec41[i] - (fConst25 * fRec36[i - 1] + fRec37[i - 1]);
				fRec36[i] = fRec36[i - 1] + fConst28 * fZec26[i];
				fZec27[i] = fRec36[i - 1] + fConst27 * fZec26[i];
				fRec37[i] = fRec37[i - 1] + fConst29 * fZec27[i];
				fZec28[i] = fConst24 * fZec27[i];
				fRec38[i] = fRec37[i - 1] + fZec28[i];
			}
			/* Post code */
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec36_perm[j35] = fRec36_tmp[vsize + j35];
			}
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec37_perm[j37] = fRec37_tmp[vsize + j37];
			}
			/* Recursive loop 62 */
			/* Pre code */
			for (int j94 = 0; j94 < 4; j94 = j94 + 1) {
				fRec91_tmp[j94] = fRec91_perm[j94];
			}
			for (int j96 = 0; j96 < 4; j96 = j96 + 1) {
				fRec92_tmp[j96] = fRec92_perm[j96];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec92[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec91[i] + -1.0f) + fZec90[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec91[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec91[i] * (1.0f - fZec91[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec90[i])))) * ((fZec90[i] > 0.0f) ? 1.0f : ((fZec90[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec90[i]) - fSlow20 : tanhf(fZec90[i])))) - (fConst18 * fRec91[i - 1] + fRec92[i - 1]);
				fRec91[i] = fRec91[i - 1] + fConst21 * fZec92[i];
				fZec93[i] = fRec91[i - 1] + fConst20 * fZec92[i];
				fRec92[i] = fRec92[i - 1] + fConst22 * fZec93[i];
				fZec94[i] = fConst23 * fZec92[i];
				fRec93[i] = fZec94[i];
			}
			/* Post code */
			for (int j95 = 0; j95 < 4; j95 = j95 + 1) {
				fRec91_perm[j95] = fRec91_tmp[vsize + j95];
			}
			for (int j97 = 0; j97 < 4; j97 = j97 + 1) {
				fRec92_perm[j97] = fRec92_tmp[vsize + j97];
			}
			/* Recursive loop 63 */
			/* Pre code */
			for (int j162 = 0; j162 < 4; j162 = j162 + 1) {
				fRec154_tmp[j162] = fRec154_perm[j162];
			}
			for (int j164 = 0; j164 < 4; j164 = j164 + 1) {
				fRec155_tmp[j164] = fRec155_perm[j164];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec162[i] = fRec159[i] - (fConst25 * fRec154[i - 1] + fRec155[i - 1]);
				fRec154[i] = fRec154[i - 1] + fConst28 * fZec162[i];
				fZec163[i] = fRec154[i - 1] + fConst27 * fZec162[i];
				fRec155[i] = fRec155[i - 1] + fConst29 * fZec163[i];
				fZec164[i] = fConst24 * fZec163[i];
				fRec156[i] = fRec155[i - 1] + fZec164[i];
			}
			/* Post code */
			for (int j163 = 0; j163 < 4; j163 = j163 + 1) {
				fRec154_perm[j163] = fRec154_tmp[vsize + j163];
			}
			for (int j165 = 0; j165 < 4; j165 = j165 + 1) {
				fRec155_perm[j165] = fRec155_tmp[vsize + j165];
			}
			/* Recursive loop 64 */
			/* Pre code */
			for (int j214 = 0; j214 < 4; j214 = j214 + 1) {
				fRec200_tmp[j214] = fRec200_perm[j214];
			}
			for (int j216 = 0; j216 < 4; j216 = j216 + 1) {
				fRec201_tmp[j216] = fRec201_perm[j216];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec220[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec219[i] + -1.0f) + fZec218[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec219[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec219[i] * (1.0f - fZec219[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec218[i])))) * ((fZec218[i] > 0.0f) ? 1.0f : ((fZec218[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec218[i]) - fSlow20 : tanhf(fZec218[i])))) - (fConst18 * fRec200[i - 1] + fRec201[i - 1]);
				fRec200[i] = fRec200[i - 1] + fConst21 * fZec220[i];
				fZec221[i] = fRec200[i - 1] + fConst20 * fZec220[i];
				fRec201[i] = fRec201[i - 1] + fConst22 * fZec221[i];
				fZec222[i] = fConst23 * fZec220[i];
				fRec202[i] = fZec222[i];
			}
			/* Post code */
			for (int j215 = 0; j215 < 4; j215 = j215 + 1) {
				fRec200_perm[j215] = fRec200_tmp[vsize + j215];
			}
			for (int j217 = 0; j217 < 4; j217 = j217 + 1) {
				fRec201_perm[j217] = fRec201_tmp[vsize + j217];
			}
			/* Vectorizable loop 65 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec29[i] = fSlow29 * fRec38[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 66 */
			/* Pre code */
			for (int j98 = 0; j98 < 4; j98 = j98 + 1) {
				fRec88_tmp[j98] = fRec88_perm[j98];
			}
			for (int j100 = 0; j100 < 4; j100 = j100 + 1) {
				fRec89_tmp[j100] = fRec89_perm[j100];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec95[i] = fRec93[i] - (fConst25 * fRec88[i - 1] + fRec89[i - 1]);
				fRec88[i] = fRec88[i - 1] + fConst28 * fZec95[i];
				fZec96[i] = fRec88[i - 1] + fConst27 * fZec95[i];
				fRec89[i] = fRec89[i - 1] + fConst29 * fZec96[i];
				fZec97[i] = fConst24 * fZec96[i];
				fRec90[i] = fRec89[i - 1] + fZec97[i];
			}
			/* Post code */
			for (int j99 = 0; j99 < 4; j99 = j99 + 1) {
				fRec88_perm[j99] = fRec88_tmp[vsize + j99];
			}
			for (int j101 = 0; j101 < 4; j101 = j101 + 1) {
				fRec89_perm[j101] = fRec89_tmp[vsize + j101];
			}
			/* Vectorizable loop 67 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec165[i] = fSlow29 * fRec156[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 68 */
			/* Pre code */
			for (int j218 = 0; j218 < 4; j218 = j218 + 1) {
				fRec197_tmp[j218] = fRec197_perm[j218];
			}
			for (int j220 = 0; j220 < 4; j220 = j220 + 1) {
				fRec198_tmp[j220] = fRec198_perm[j220];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec223[i] = fRec202[i] - (fConst25 * fRec197[i - 1] + fRec198[i - 1]);
				fRec197[i] = fRec197[i - 1] + fConst28 * fZec223[i];
				fZec224[i] = fRec197[i - 1] + fConst27 * fZec223[i];
				fRec198[i] = fRec198[i - 1] + fConst29 * fZec224[i];
				fZec225[i] = fConst24 * fZec224[i];
				fRec199[i] = fRec198[i - 1] + fZec225[i];
			}
			/* Post code */
			for (int j219 = 0; j219 < 4; j219 = j219 + 1) {
				fRec197_perm[j219] = fRec197_tmp[vsize + j219];
			}
			for (int j221 = 0; j221 < 4; j221 = j221 + 1) {
				fRec198_perm[j221] = fRec198_tmp[vsize + j221];
			}
			/* Vectorizable loop 69 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec30[i] = mlczerov_faustpower2_f(fZec29[i]);
			}
			/* Recursive loop 70 */
			/* Pre code */
			for (int j50 = 0; j50 < 4; j50 = j50 + 1) {
				fRec61_tmp[j50] = fRec61_perm[j50];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec61[i] = fSlow33 + fConst2 * fRec61[i - 1];
			}
			/* Post code */
			for (int j51 = 0; j51 < 4; j51 = j51 + 1) {
				fRec61_perm[j51] = fRec61_tmp[vsize + j51];
			}
			/* Vectorizable loop 71 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec98[i] = fRec90[i] * fZec10[i];
			}
			/* Vectorizable loop 72 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec166[i] = mlczerov_faustpower2_f(fZec165[i]);
			}
			/* Vectorizable loop 73 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec226[i] = fRec199[i] * fZec10[i];
			}
			/* Recursive loop 74 */
			/* Pre code */
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec33_tmp[j38] = fRec33_perm[j38];
			}
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec34_tmp[j40] = fRec34_perm[j40];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec31[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec30[i] + -1.0f) + fZec29[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec30[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec30[i] * (1.0f - fZec30[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec29[i])))) * ((fZec29[i] > 0.0f) ? 1.0f : ((fZec29[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec29[i]) - fSlow20 : tanhf(fZec29[i])))) - (fConst18 * fRec33[i - 1] + fRec34[i - 1]);
				fRec33[i] = fRec33[i - 1] + fConst21 * fZec31[i];
				fZec32[i] = fRec33[i - 1] + fConst20 * fZec31[i];
				fRec34[i] = fRec34[i - 1] + fConst22 * fZec32[i];
				fZec33[i] = fConst23 * fZec31[i];
				fRec35[i] = fZec33[i];
			}
			/* Post code */
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec33_perm[j39] = fRec33_tmp[vsize + j39];
			}
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec34_perm[j41] = fRec34_tmp[vsize + j41];
			}
			/* Vectorizable loop 75 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec40[i] = 0.46f * fZec17[i];
			}
			/* Vectorizable loop 76 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec45[i] = std::pow(1e+01f, 0.05f * fRec61[i]);
			}
			/* Vectorizable loop 77 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec99[i] = 0.46f * fZec98[i];
			}
			/* Vectorizable loop 78 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec100[i] = mlczerov_faustpower2_f(fRec90[i]) * fZec15[i];
			}
			/* Recursive loop 79 */
			/* Pre code */
			for (int j166 = 0; j166 < 4; j166 = j166 + 1) {
				fRec151_tmp[j166] = fRec151_perm[j166];
			}
			for (int j168 = 0; j168 < 4; j168 = j168 + 1) {
				fRec152_tmp[j168] = fRec152_perm[j168];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec167[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec166[i] + -1.0f) + fZec165[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec166[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec166[i] * (1.0f - fZec166[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec165[i])))) * ((fZec165[i] > 0.0f) ? 1.0f : ((fZec165[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec165[i]) - fSlow20 : tanhf(fZec165[i])))) - (fConst18 * fRec151[i - 1] + fRec152[i - 1]);
				fRec151[i] = fRec151[i - 1] + fConst21 * fZec167[i];
				fZec168[i] = fRec151[i - 1] + fConst20 * fZec167[i];
				fRec152[i] = fRec152[i - 1] + fConst22 * fZec168[i];
				fZec169[i] = fConst23 * fZec167[i];
				fRec153[i] = fZec169[i];
			}
			/* Post code */
			for (int j167 = 0; j167 < 4; j167 = j167 + 1) {
				fRec151_perm[j167] = fRec151_tmp[vsize + j167];
			}
			for (int j169 = 0; j169 < 4; j169 = j169 + 1) {
				fRec152_perm[j169] = fRec152_tmp[vsize + j169];
			}
			/* Vectorizable loop 80 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec227[i] = 0.46f * fZec226[i];
			}
			/* Vectorizable loop 81 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec228[i] = mlczerov_faustpower2_f(fRec199[i]) * fZec15[i];
			}
			/* Recursive loop 82 */
			/* Pre code */
			for (int j42 = 0; j42 < 4; j42 = j42 + 1) {
				fRec30_tmp[j42] = fRec30_perm[j42];
			}
			for (int j44 = 0; j44 < 4; j44 = j44 + 1) {
				fRec31_tmp[j44] = fRec31_perm[j44];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec34[i] = fRec35[i] - (fConst25 * fRec30[i - 1] + fRec31[i - 1]);
				fRec30[i] = fRec30[i - 1] + fConst28 * fZec34[i];
				fZec35[i] = fRec30[i - 1] + fConst27 * fZec34[i];
				fRec31[i] = fRec31[i - 1] + fConst29 * fZec35[i];
				fZec36[i] = fConst24 * fZec35[i];
				fRec32[i] = fRec31[i - 1] + fZec36[i];
			}
			/* Post code */
			for (int j43 = 0; j43 < 4; j43 = j43 + 1) {
				fRec30_perm[j43] = fRec30_tmp[vsize + j43];
			}
			for (int j45 = 0; j45 < 4; j45 = j45 + 1) {
				fRec31_perm[j45] = fRec31_tmp[vsize + j45];
			}
			/* Vectorizable loop 83 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec46[i] = std::sqrt(fZec45[i]);
			}
			/* Recursive loop 84 */
			/* Pre code */
			for (int j56 = 0; j56 < 4; j56 = j56 + 1) {
				fRec62_tmp[j56] = fRec62_perm[j56];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec62[i] = fSlow34 + fConst2 * fRec62[i - 1];
			}
			/* Post code */
			for (int j57 = 0; j57 < 4; j57 = j57 + 1) {
				fRec62_perm[j57] = fRec62_tmp[vsize + j57];
			}
			/* Recursive loop 85 */
			/* Pre code */
			for (int j62 = 0; j62 < 4; j62 = j62 + 1) {
				fRec63_tmp[j62] = fRec63_perm[j62];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec63[i] = fSlow35 + fConst2 * fRec63[i - 1];
			}
			/* Post code */
			for (int j63 = 0; j63 < 4; j63 = j63 + 1) {
				fRec63_perm[j63] = fRec63_tmp[vsize + j63];
			}
			/* Recursive loop 86 */
			/* Pre code */
			for (int j102 = 0; j102 < 4; j102 = j102 + 1) {
				fRec83_tmp[j102] = fRec83_perm[j102];
			}
			for (int j104 = 0; j104 < 4; j104 = j104 + 1) {
				fRec84_tmp[j104] = fRec84_perm[j104];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec101[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec100[i] + -1.0f) + fZec98[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec100[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec100[i] * (0.35819647f * fZec100[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec99[i])))) * ((fZec99[i] > 0.0f) ? 1.0f : ((fZec99[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec99[i]) - fSlow20 : tanhf(fZec99[i])))) - (fConst31 * fRec83[i - 1] + fRec84[i - 1]);
				fRec83[i] = fRec83[i - 1] + fConst34 * fZec101[i];
				fZec102[i] = fRec83[i - 1] + fConst33 * fZec101[i];
				fRec84[i] = fRec84[i - 1] + fConst35 * fZec102[i];
				fZec103[i] = fConst30 * fZec102[i];
				fRec85[i] = fRec84[i - 1] + fZec103[i];
				fZec104[i] = fConst36 * fZec101[i];
				fRec86[i] = fZec104[i];
				fRec87[i] = fZec102[i];
			}
			/* Post code */
			for (int j103 = 0; j103 < 4; j103 = j103 + 1) {
				fRec83_perm[j103] = fRec83_tmp[vsize + j103];
			}
			for (int j105 = 0; j105 < 4; j105 = j105 + 1) {
				fRec84_perm[j105] = fRec84_tmp[vsize + j105];
			}
			/* Recursive loop 87 */
			/* Pre code */
			for (int j170 = 0; j170 < 4; j170 = j170 + 1) {
				fRec148_tmp[j170] = fRec148_perm[j170];
			}
			for (int j172 = 0; j172 < 4; j172 = j172 + 1) {
				fRec149_tmp[j172] = fRec149_perm[j172];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec170[i] = fRec153[i] - (fConst25 * fRec148[i - 1] + fRec149[i - 1]);
				fRec148[i] = fRec148[i - 1] + fConst28 * fZec170[i];
				fZec171[i] = fRec148[i - 1] + fConst27 * fZec170[i];
				fRec149[i] = fRec149[i - 1] + fConst29 * fZec171[i];
				fZec172[i] = fConst24 * fZec171[i];
				fRec150[i] = fRec149[i - 1] + fZec172[i];
			}
			/* Post code */
			for (int j171 = 0; j171 < 4; j171 = j171 + 1) {
				fRec148_perm[j171] = fRec148_tmp[vsize + j171];
			}
			for (int j173 = 0; j173 < 4; j173 = j173 + 1) {
				fRec149_perm[j173] = fRec149_tmp[vsize + j173];
			}
			/* Recursive loop 88 */
			/* Pre code */
			for (int j222 = 0; j222 < 4; j222 = j222 + 1) {
				fRec192_tmp[j222] = fRec192_perm[j222];
			}
			for (int j224 = 0; j224 < 4; j224 = j224 + 1) {
				fRec193_tmp[j224] = fRec193_perm[j224];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec229[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec228[i] + -1.0f) + fZec226[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec228[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec228[i] * (0.35819647f * fZec228[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec227[i])))) * ((fZec227[i] > 0.0f) ? 1.0f : ((fZec227[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec227[i]) - fSlow20 : tanhf(fZec227[i])))) - (fConst31 * fRec192[i - 1] + fRec193[i - 1]);
				fRec192[i] = fRec192[i - 1] + fConst34 * fZec229[i];
				fZec230[i] = fRec192[i - 1] + fConst33 * fZec229[i];
				fRec193[i] = fRec193[i - 1] + fConst35 * fZec230[i];
				fZec231[i] = fConst30 * fZec230[i];
				fRec194[i] = fRec193[i - 1] + fZec231[i];
				fZec232[i] = fConst36 * fZec229[i];
				fRec195[i] = fZec232[i];
				fRec196[i] = fZec230[i];
			}
			/* Post code */
			for (int j223 = 0; j223 < 4; j223 = j223 + 1) {
				fRec192_perm[j223] = fRec192_tmp[vsize + j223];
			}
			for (int j225 = 0; j225 < 4; j225 = j225 + 1) {
				fRec193_perm[j225] = fRec193_tmp[vsize + j225];
			}
			/* Vectorizable loop 89 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec37[i] = fRec32[i] * fZec10[i];
			}
			/* Vectorizable loop 90 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec51[i] = std::pow(1e+01f, 0.05f * (fRec62[i] + -2.5f));
			}
			/* Vectorizable loop 91 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec56[i] = std::pow(1e+01f, 0.05f * fRec63[i]);
			}
			/* Recursive loop 92 */
			/* Pre code */
			for (int j106 = 0; j106 < 4; j106 = j106 + 1) {
				fRec79_tmp[j106] = fRec79_perm[j106];
			}
			for (int j108 = 0; j108 < 4; j108 = j108 + 1) {
				fRec80_tmp[j108] = fRec80_perm[j108];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec105[i] = fRec86[i] + fRec85[i] * fZec45[i] + 1.4144272f * fRec87[i] * fZec46[i] - (fConst38 * fRec79[i - 1] + fRec80[i - 1]);
				fRec79[i] = fRec79[i - 1] + fConst41 * fZec105[i];
				fZec106[i] = fRec79[i - 1] + fConst40 * fZec105[i];
				fRec80[i] = fRec80[i - 1] + fConst42 * fZec106[i];
				fRec81[i] = fZec106[i];
				fZec107[i] = fConst43 * fZec105[i];
				fZec108[i] = fConst37 * fZec106[i];
				fRec82[i] = fZec108[i] + fRec80[i - 1] + fZec107[i];
			}
			/* Post code */
			for (int j107 = 0; j107 < 4; j107 = j107 + 1) {
				fRec79_perm[j107] = fRec79_tmp[vsize + j107];
			}
			for (int j109 = 0; j109 < 4; j109 = j109 + 1) {
				fRec80_perm[j109] = fRec80_tmp[vsize + j109];
			}
			/* Vectorizable loop 93 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec173[i] = fRec150[i] * fZec10[i];
			}
			/* Recursive loop 94 */
			/* Pre code */
			for (int j226 = 0; j226 < 4; j226 = j226 + 1) {
				fRec188_tmp[j226] = fRec188_perm[j226];
			}
			for (int j228 = 0; j228 < 4; j228 = j228 + 1) {
				fRec189_tmp[j228] = fRec189_perm[j228];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec233[i] = fRec195[i] + fRec194[i] * fZec45[i] + 1.4144272f * fRec196[i] * fZec46[i] - (fConst38 * fRec188[i - 1] + fRec189[i - 1]);
				fRec188[i] = fRec188[i - 1] + fConst41 * fZec233[i];
				fZec234[i] = fRec188[i - 1] + fConst40 * fZec233[i];
				fRec189[i] = fRec189[i - 1] + fConst42 * fZec234[i];
				fRec190[i] = fZec234[i];
				fZec235[i] = fConst43 * fZec233[i];
				fZec236[i] = fConst37 * fZec234[i];
				fRec191[i] = fZec236[i] + fRec189[i - 1] + fZec235[i];
			}
			/* Post code */
			for (int j227 = 0; j227 < 4; j227 = j227 + 1) {
				fRec188_perm[j227] = fRec188_tmp[vsize + j227];
			}
			for (int j229 = 0; j229 < 4; j229 = j229 + 1) {
				fRec189_perm[j229] = fRec189_tmp[vsize + j229];
			}
			/* Vectorizable loop 95 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec38[i] = 0.46f * fZec37[i];
			}
			/* Vectorizable loop 96 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec39[i] = mlczerov_faustpower2_f(fRec32[i]) * fZec15[i];
			}
			/* Vectorizable loop 97 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec57[i] = std::sqrt(fZec56[i]);
			}
			/* Recursive loop 98 */
			/* Pre code */
			for (int j110 = 0; j110 < 4; j110 = j110 + 1) {
				fRec74_tmp[j110] = fRec74_perm[j110];
			}
			for (int j112 = 0; j112 < 4; j112 = j112 + 1) {
				fRec75_tmp[j112] = fRec75_perm[j112];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec109[i] = fRec82[i] + fRec81[i] * fZec51[i] - (fConst45 * fRec74[i - 1] + fRec75[i - 1]);
				fRec74[i] = fRec74[i - 1] + fConst48 * fZec109[i];
				fZec110[i] = fRec74[i - 1] + fConst47 * fZec109[i];
				fRec75[i] = fRec75[i - 1] + fConst49 * fZec110[i];
				fZec111[i] = fConst44 * fZec110[i];
				fRec76[i] = fRec75[i - 1] + fZec111[i];
				fZec112[i] = fConst50 * fZec109[i];
				fRec77[i] = fZec112[i];
				fRec78[i] = fZec110[i];
			}
			/* Post code */
			for (int j111 = 0; j111 < 4; j111 = j111 + 1) {
				fRec74_perm[j111] = fRec74_tmp[vsize + j111];
			}
			for (int j113 = 0; j113 < 4; j113 = j113 + 1) {
				fRec75_perm[j113] = fRec75_tmp[vsize + j113];
			}
			/* Vectorizable loop 99 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec174[i] = 0.46f * fZec173[i];
			}
			/* Vectorizable loop 100 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec175[i] = mlczerov_faustpower2_f(fRec150[i]) * fZec15[i];
			}
			/* Recursive loop 101 */
			/* Pre code */
			for (int j230 = 0; j230 < 4; j230 = j230 + 1) {
				fRec183_tmp[j230] = fRec183_perm[j230];
			}
			for (int j232 = 0; j232 < 4; j232 = j232 + 1) {
				fRec184_tmp[j232] = fRec184_perm[j232];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec237[i] = fRec191[i] + fRec190[i] * fZec51[i] - (fConst45 * fRec183[i - 1] + fRec184[i - 1]);
				fRec183[i] = fRec183[i - 1] + fConst48 * fZec237[i];
				fZec238[i] = fRec183[i - 1] + fConst47 * fZec237[i];
				fRec184[i] = fRec184[i - 1] + fConst49 * fZec238[i];
				fZec239[i] = fConst44 * fZec238[i];
				fRec185[i] = fRec184[i - 1] + fZec239[i];
				fZec240[i] = fConst50 * fZec237[i];
				fRec186[i] = fZec240[i];
				fRec187[i] = fZec238[i];
			}
			/* Post code */
			for (int j231 = 0; j231 < 4; j231 = j231 + 1) {
				fRec183_perm[j231] = fRec183_tmp[vsize + j231];
			}
			for (int j233 = 0; j233 < 4; j233 = j233 + 1) {
				fRec184_perm[j233] = fRec184_tmp[vsize + j233];
			}
			/* Recursive loop 102 */
			/* Pre code */
			for (int j46 = 0; j46 < 4; j46 = j46 + 1) {
				fRec25_tmp[j46] = fRec25_perm[j46];
			}
			for (int j48 = 0; j48 < 4; j48 = j48 + 1) {
				fRec26_tmp[j48] = fRec26_perm[j48];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec41[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec39[i] + -1.0f) + fZec37[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec39[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec39[i] * (0.35819647f * fZec39[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec38[i])))) * ((fZec38[i] > 0.0f) ? 1.0f : ((fZec38[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec38[i]) - fSlow20 : tanhf(fZec38[i])))) - (fConst31 * fRec25[i - 1] + fRec26[i - 1]);
				fRec25[i] = fRec25[i - 1] + fConst34 * fZec41[i];
				fZec42[i] = fRec25[i - 1] + fConst33 * fZec41[i];
				fRec26[i] = fRec26[i - 1] + fConst35 * fZec42[i];
				fZec43[i] = fConst30 * fZec42[i];
				fRec27[i] = fRec26[i - 1] + fZec43[i];
				fZec44[i] = fConst36 * fZec41[i];
				fRec28[i] = fZec44[i];
				fRec29[i] = fZec42[i];
			}
			/* Post code */
			for (int j47 = 0; j47 < 4; j47 = j47 + 1) {
				fRec25_perm[j47] = fRec25_tmp[vsize + j47];
			}
			for (int j49 = 0; j49 < 4; j49 = j49 + 1) {
				fRec26_perm[j49] = fRec26_tmp[vsize + j49];
			}
			/* Vectorizable loop 103 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec113[i] = fRec76[i] + fRec77[i] * fZec56[i] + 1.4144272f * fRec78[i] * fZec57[i];
			}
			/* Recursive loop 104 */
			/* Pre code */
			for (int j174 = 0; j174 < 4; j174 = j174 + 1) {
				fRec143_tmp[j174] = fRec143_perm[j174];
			}
			for (int j176 = 0; j176 < 4; j176 = j176 + 1) {
				fRec144_tmp[j176] = fRec144_perm[j176];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec176[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec175[i] + -1.0f) + fZec173[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec175[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec175[i] * (0.35819647f * fZec175[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec174[i])))) * ((fZec174[i] > 0.0f) ? 1.0f : ((fZec174[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec174[i]) - fSlow20 : tanhf(fZec174[i])))) - (fConst31 * fRec143[i - 1] + fRec144[i - 1]);
				fRec143[i] = fRec143[i - 1] + fConst34 * fZec176[i];
				fZec177[i] = fRec143[i - 1] + fConst33 * fZec176[i];
				fRec144[i] = fRec144[i - 1] + fConst35 * fZec177[i];
				fZec178[i] = fConst30 * fZec177[i];
				fRec145[i] = fRec144[i - 1] + fZec178[i];
				fZec179[i] = fConst36 * fZec176[i];
				fRec146[i] = fZec179[i];
				fRec147[i] = fZec177[i];
			}
			/* Post code */
			for (int j175 = 0; j175 < 4; j175 = j175 + 1) {
				fRec143_perm[j175] = fRec143_tmp[vsize + j175];
			}
			for (int j177 = 0; j177 < 4; j177 = j177 + 1) {
				fRec144_perm[j177] = fRec144_tmp[vsize + j177];
			}
			/* Vectorizable loop 105 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec241[i] = fRec185[i] + fRec186[i] * fZec56[i] + 1.4144272f * fRec187[i] * fZec57[i];
			}
			/* Recursive loop 106 */
			/* Pre code */
			for (int j52 = 0; j52 < 4; j52 = j52 + 1) {
				fRec21_tmp[j52] = fRec21_perm[j52];
			}
			for (int j54 = 0; j54 < 4; j54 = j54 + 1) {
				fRec22_tmp[j54] = fRec22_perm[j54];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec47[i] = fRec28[i] + fRec27[i] * fZec45[i] + 1.4144272f * fRec29[i] * fZec46[i] - (fConst38 * fRec21[i - 1] + fRec22[i - 1]);
				fRec21[i] = fRec21[i - 1] + fConst41 * fZec47[i];
				fZec48[i] = fRec21[i - 1] + fConst40 * fZec47[i];
				fRec22[i] = fRec22[i - 1] + fConst42 * fZec48[i];
				fRec23[i] = fZec48[i];
				fZec49[i] = fConst43 * fZec47[i];
				fZec50[i] = fConst37 * fZec48[i];
				fRec24[i] = fZec50[i] + fRec22[i - 1] + fZec49[i];
			}
			/* Post code */
			for (int j53 = 0; j53 < 4; j53 = j53 + 1) {
				fRec21_perm[j53] = fRec21_tmp[vsize + j53];
			}
			for (int j55 = 0; j55 < 4; j55 = j55 + 1) {
				fRec22_perm[j55] = fRec22_tmp[vsize + j55];
			}
			/* Recursive loop 107 */
			/* Pre code */
			for (int j64 = 0; j64 < 4; j64 = j64 + 1) {
				fRec64_tmp[j64] = fRec64_perm[j64];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec64[i] = fSlow36 + fConst2 * fRec64[i - 1];
			}
			/* Post code */
			for (int j65 = 0; j65 < 4; j65 = j65 + 1) {
				fRec64_perm[j65] = fRec64_tmp[vsize + j65];
			}
			/* Recursive loop 108 */
			/* Pre code */
			for (int j114 = 0; j114 < 4; j114 = j114 + 1) {
				fRec112_tmp[j114] = fRec112_perm[j114];
			}
			for (int j116 = 0; j116 < 4; j116 = j116 + 1) {
				fRec113_tmp[j116] = fRec113_perm[j116];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec114[i] = mlczerov_faustpower2_f(fZec113[i]) - (fConst52 * fRec112[i - 1] + fRec113[i - 1]);
				fRec112[i] = fRec112[i - 1] + fConst55 * fZec114[i];
				fZec115[i] = fRec112[i - 1] + fConst54 * fZec114[i];
				fRec113[i] = fRec113[i - 1] + fConst56 * fZec115[i];
				fZec116[i] = fConst51 * fZec115[i];
				fRec114[i] = fRec113[i - 1] + fZec116[i];
			}
			/* Post code */
			for (int j115 = 0; j115 < 4; j115 = j115 + 1) {
				fRec112_perm[j115] = fRec112_tmp[vsize + j115];
			}
			for (int j117 = 0; j117 < 4; j117 = j117 + 1) {
				fRec113_perm[j117] = fRec113_tmp[vsize + j117];
			}
			/* Recursive loop 109 */
			/* Pre code */
			for (int j178 = 0; j178 < 4; j178 = j178 + 1) {
				fRec139_tmp[j178] = fRec139_perm[j178];
			}
			for (int j180 = 0; j180 < 4; j180 = j180 + 1) {
				fRec140_tmp[j180] = fRec140_perm[j180];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec180[i] = fRec146[i] + fRec145[i] * fZec45[i] + 1.4144272f * fRec147[i] * fZec46[i] - (fConst38 * fRec139[i - 1] + fRec140[i - 1]);
				fRec139[i] = fRec139[i - 1] + fConst41 * fZec180[i];
				fZec181[i] = fRec139[i - 1] + fConst40 * fZec180[i];
				fRec140[i] = fRec140[i - 1] + fConst42 * fZec181[i];
				fRec141[i] = fZec181[i];
				fZec182[i] = fConst43 * fZec180[i];
				fZec183[i] = fConst37 * fZec181[i];
				fRec142[i] = fZec183[i] + fRec140[i - 1] + fZec182[i];
			}
			/* Post code */
			for (int j179 = 0; j179 < 4; j179 = j179 + 1) {
				fRec139_perm[j179] = fRec139_tmp[vsize + j179];
			}
			for (int j181 = 0; j181 < 4; j181 = j181 + 1) {
				fRec140_perm[j181] = fRec140_tmp[vsize + j181];
			}
			/* Recursive loop 110 */
			/* Pre code */
			for (int j234 = 0; j234 < 4; j234 = j234 + 1) {
				fRec221_tmp[j234] = fRec221_perm[j234];
			}
			for (int j236 = 0; j236 < 4; j236 = j236 + 1) {
				fRec222_tmp[j236] = fRec222_perm[j236];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec242[i] = mlczerov_faustpower2_f(fZec241[i]) - (fConst52 * fRec221[i - 1] + fRec222[i - 1]);
				fRec221[i] = fRec221[i - 1] + fConst55 * fZec242[i];
				fZec243[i] = fRec221[i - 1] + fConst54 * fZec242[i];
				fRec222[i] = fRec222[i - 1] + fConst56 * fZec243[i];
				fZec244[i] = fConst51 * fZec243[i];
				fRec223[i] = fRec222[i - 1] + fZec244[i];
			}
			/* Post code */
			for (int j235 = 0; j235 < 4; j235 = j235 + 1) {
				fRec221_perm[j235] = fRec221_tmp[vsize + j235];
			}
			for (int j237 = 0; j237 < 4; j237 = j237 + 1) {
				fRec222_perm[j237] = fRec222_tmp[vsize + j237];
			}
			/* Recursive loop 111 */
			/* Pre code */
			for (int j58 = 0; j58 < 4; j58 = j58 + 1) {
				fRec16_tmp[j58] = fRec16_perm[j58];
			}
			for (int j60 = 0; j60 < 4; j60 = j60 + 1) {
				fRec17_tmp[j60] = fRec17_perm[j60];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec52[i] = fRec24[i] + fRec23[i] * fZec51[i] - (fConst45 * fRec16[i - 1] + fRec17[i - 1]);
				fRec16[i] = fRec16[i - 1] + fConst48 * fZec52[i];
				fZec53[i] = fRec16[i - 1] + fConst47 * fZec52[i];
				fRec17[i] = fRec17[i - 1] + fConst49 * fZec53[i];
				fZec54[i] = fConst44 * fZec53[i];
				fRec18[i] = fRec17[i - 1] + fZec54[i];
				fZec55[i] = fConst50 * fZec52[i];
				fRec19[i] = fZec55[i];
				fRec20[i] = fZec53[i];
			}
			/* Post code */
			for (int j59 = 0; j59 < 4; j59 = j59 + 1) {
				fRec16_perm[j59] = fRec16_tmp[vsize + j59];
			}
			for (int j61 = 0; j61 < 4; j61 = j61 + 1) {
				fRec17_perm[j61] = fRec17_tmp[vsize + j61];
			}
			/* Recursive loop 112 */
			/* Pre code */
			for (int j118 = 0; j118 < 4; j118 = j118 + 1) {
				fRec70_tmp[j118] = fRec70_perm[j118];
			}
			for (int j120 = 0; j120 < 4; j120 = j120 + 1) {
				fRec71_tmp[j120] = fRec71_perm[j120];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec117[i] = fSlow38 * fZec113[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec114[i])) - (fConst58 * fRec70[i - 1] + fRec71[i - 1]);
				fRec70[i] = fRec70[i - 1] + fConst61 * fZec117[i];
				fZec118[i] = fRec70[i - 1] + fConst60 * fZec117[i];
				fRec71[i] = fRec71[i - 1] + fConst62 * fZec118[i];
				fRec72[i] = fZec118[i];
				fZec119[i] = fConst63 * fZec117[i];
				fZec120[i] = fConst57 * fZec118[i];
				fRec73[i] = fZec120[i] + fRec71[i - 1] + fZec119[i];
			}
			/* Post code */
			for (int j119 = 0; j119 < 4; j119 = j119 + 1) {
				fRec70_perm[j119] = fRec70_tmp[vsize + j119];
			}
			for (int j121 = 0; j121 < 4; j121 = j121 + 1) {
				fRec71_perm[j121] = fRec71_tmp[vsize + j121];
			}
			/* Recursive loop 113 */
			/* Pre code */
			for (int j182 = 0; j182 < 4; j182 = j182 + 1) {
				fRec134_tmp[j182] = fRec134_perm[j182];
			}
			for (int j184 = 0; j184 < 4; j184 = j184 + 1) {
				fRec135_tmp[j184] = fRec135_perm[j184];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec184[i] = fRec142[i] + fRec141[i] * fZec51[i] - (fConst45 * fRec134[i - 1] + fRec135[i - 1]);
				fRec134[i] = fRec134[i - 1] + fConst48 * fZec184[i];
				fZec185[i] = fRec134[i - 1] + fConst47 * fZec184[i];
				fRec135[i] = fRec135[i - 1] + fConst49 * fZec185[i];
				fZec186[i] = fConst44 * fZec185[i];
				fRec136[i] = fRec135[i - 1] + fZec186[i];
				fZec187[i] = fConst50 * fZec184[i];
				fRec137[i] = fZec187[i];
				fRec138[i] = fZec185[i];
			}
			/* Post code */
			for (int j183 = 0; j183 < 4; j183 = j183 + 1) {
				fRec134_perm[j183] = fRec134_tmp[vsize + j183];
			}
			for (int j185 = 0; j185 < 4; j185 = j185 + 1) {
				fRec135_perm[j185] = fRec135_tmp[vsize + j185];
			}
			/* Recursive loop 114 */
			/* Pre code */
			for (int j238 = 0; j238 < 4; j238 = j238 + 1) {
				fRec179_tmp[j238] = fRec179_perm[j238];
			}
			for (int j240 = 0; j240 < 4; j240 = j240 + 1) {
				fRec180_tmp[j240] = fRec180_perm[j240];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec245[i] = fSlow38 * fZec241[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec223[i])) - (fConst58 * fRec179[i - 1] + fRec180[i - 1]);
				fRec179[i] = fRec179[i - 1] + fConst61 * fZec245[i];
				fZec246[i] = fRec179[i - 1] + fConst60 * fZec245[i];
				fRec180[i] = fRec180[i - 1] + fConst62 * fZec246[i];
				fRec181[i] = fZec246[i];
				fZec247[i] = fConst63 * fZec245[i];
				fZec248[i] = fConst57 * fZec246[i];
				fRec182[i] = fZec248[i] + fRec180[i - 1] + fZec247[i];
			}
			/* Post code */
			for (int j239 = 0; j239 < 4; j239 = j239 + 1) {
				fRec179_perm[j239] = fRec179_tmp[vsize + j239];
			}
			for (int j241 = 0; j241 < 4; j241 = j241 + 1) {
				fRec180_perm[j241] = fRec180_tmp[vsize + j241];
			}
			/* Vectorizable loop 115 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec58[i] = fRec18[i] + fRec19[i] * fZec56[i] + 1.4144272f * fRec20[i] * fZec57[i];
			}
			/* Vectorizable loop 116 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec121[i] = fRec73[i] + fSlow39 * fRec72[i];
			}
			/* Vectorizable loop 117 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec188[i] = fRec136[i] + fRec137[i] * fZec56[i] + 1.4144272f * fRec138[i] * fZec57[i];
			}
			/* Vectorizable loop 118 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec249[i] = fRec182[i] + fSlow39 * fRec181[i];
			}
			/* Recursive loop 119 */
			/* Pre code */
			for (int j66 = 0; j66 < 4; j66 = j66 + 1) {
				fRec65_tmp[j66] = fRec65_perm[j66];
			}
			for (int j68 = 0; j68 < 4; j68 = j68 + 1) {
				fRec66_tmp[j68] = fRec66_perm[j68];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec59[i] = mlczerov_faustpower2_f(fZec58[i]) - (fConst52 * fRec65[i - 1] + fRec66[i - 1]);
				fRec65[i] = fRec65[i - 1] + fConst55 * fZec59[i];
				fZec60[i] = fRec65[i - 1] + fConst54 * fZec59[i];
				fRec66[i] = fRec66[i - 1] + fConst56 * fZec60[i];
				fZec61[i] = fConst51 * fZec60[i];
				fRec67[i] = fRec66[i - 1] + fZec61[i];
			}
			/* Post code */
			for (int j67 = 0; j67 < 4; j67 = j67 + 1) {
				fRec65_perm[j67] = fRec65_tmp[vsize + j67];
			}
			for (int j69 = 0; j69 < 4; j69 = j69 + 1) {
				fRec66_perm[j69] = fRec66_tmp[vsize + j69];
			}
			/* Vectorizable loop 120 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec122[i] = mlczerov_faustpower2_f(fZec121[i]);
			}
			/* Recursive loop 121 */
			/* Pre code */
			for (int j186 = 0; j186 < 4; j186 = j186 + 1) {
				fRec174_tmp[j186] = fRec174_perm[j186];
			}
			for (int j188 = 0; j188 < 4; j188 = j188 + 1) {
				fRec175_tmp[j188] = fRec175_perm[j188];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec189[i] = mlczerov_faustpower2_f(fZec188[i]) - (fConst52 * fRec174[i - 1] + fRec175[i - 1]);
				fRec174[i] = fRec174[i - 1] + fConst55 * fZec189[i];
				fZec190[i] = fRec174[i - 1] + fConst54 * fZec189[i];
				fRec175[i] = fRec175[i - 1] + fConst56 * fZec190[i];
				fZec191[i] = fConst51 * fZec190[i];
				fRec176[i] = fRec175[i - 1] + fZec191[i];
			}
			/* Post code */
			for (int j187 = 0; j187 < 4; j187 = j187 + 1) {
				fRec174_perm[j187] = fRec174_tmp[vsize + j187];
			}
			for (int j189 = 0; j189 < 4; j189 = j189 + 1) {
				fRec175_perm[j189] = fRec175_tmp[vsize + j189];
			}
			/* Vectorizable loop 122 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec250[i] = mlczerov_faustpower2_f(fZec249[i]);
			}
			/* Recursive loop 123 */
			/* Pre code */
			for (int j70 = 0; j70 < 4; j70 = j70 + 1) {
				fRec12_tmp[j70] = fRec12_perm[j70];
			}
			for (int j72 = 0; j72 < 4; j72 = j72 + 1) {
				fRec13_tmp[j72] = fRec13_perm[j72];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec62[i] = fSlow38 * fZec58[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec67[i])) - (fConst58 * fRec12[i - 1] + fRec13[i - 1]);
				fRec12[i] = fRec12[i - 1] + fConst61 * fZec62[i];
				fZec63[i] = fRec12[i - 1] + fConst60 * fZec62[i];
				fRec13[i] = fRec13[i - 1] + fConst62 * fZec63[i];
				fRec14[i] = fZec63[i];
				fZec64[i] = fConst63 * fZec62[i];
				fZec65[i] = fConst57 * fZec63[i];
				fRec15[i] = fZec65[i] + fRec13[i - 1] + fZec64[i];
			}
			/* Post code */
			for (int j71 = 0; j71 < 4; j71 = j71 + 1) {
				fRec12_perm[j71] = fRec12_tmp[vsize + j71];
			}
			for (int j73 = 0; j73 < 4; j73 = j73 + 1) {
				fRec13_perm[j73] = fRec13_tmp[vsize + j73];
			}
			/* Vectorizable loop 124 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec123[i] = ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec122[i] + -1.0f) + fZec121[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec122[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec122[i] * (1.0f - fZec122[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec121[i])))) * ((fZec121[i] > 0.0f) ? 1.0f : ((fZec121[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec121[i]) - fSlow20 : tanhf(fZec121[i]))));
			}
			/* Recursive loop 125 */
			/* Pre code */
			for (int j190 = 0; j190 < 4; j190 = j190 + 1) {
				fRec130_tmp[j190] = fRec130_perm[j190];
			}
			for (int j192 = 0; j192 < 4; j192 = j192 + 1) {
				fRec131_tmp[j192] = fRec131_perm[j192];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec192[i] = fSlow38 * fZec188[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec176[i])) - (fConst58 * fRec130[i - 1] + fRec131[i - 1]);
				fRec130[i] = fRec130[i - 1] + fConst61 * fZec192[i];
				fZec193[i] = fRec130[i - 1] + fConst60 * fZec192[i];
				fRec131[i] = fRec131[i - 1] + fConst62 * fZec193[i];
				fRec132[i] = fZec193[i];
				fZec194[i] = fConst63 * fZec192[i];
				fZec195[i] = fConst57 * fZec193[i];
				fRec133[i] = fZec195[i] + fRec131[i - 1] + fZec194[i];
			}
			/* Post code */
			for (int j191 = 0; j191 < 4; j191 = j191 + 1) {
				fRec130_perm[j191] = fRec130_tmp[vsize + j191];
			}
			for (int j193 = 0; j193 < 4; j193 = j193 + 1) {
				fRec131_perm[j193] = fRec131_tmp[vsize + j193];
			}
			/* Vectorizable loop 126 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec251[i] = ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec250[i] + -1.0f) + fZec249[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec250[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec250[i] * (1.0f - fZec250[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec249[i])))) * ((fZec249[i] > 0.0f) ? 1.0f : ((fZec249[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec249[i]) - fSlow20 : tanhf(fZec249[i]))));
			}
			/* Recursive loop 127 */
			/* Pre code */
			for (int j122 = 0; j122 < 4; j122 = j122 + 1) {
				fRec69_tmp[j122] = fRec69_perm[j122];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec69[i] = std::max<float>(0.995f * fRec69[i - 1], std::fabs(fSlow40 * fZec123[i]));
			}
			/* Post code */
			for (int j123 = 0; j123 < 4; j123 = j123 + 1) {
				fRec69_perm[j123] = fRec69_tmp[vsize + j123];
			}
			/* Vectorizable loop 128 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec124[i] = fRec15[i] + fSlow39 * fRec14[i];
			}
			/* Recursive loop 129 */
			/* Pre code */
			for (int j130 = 0; j130 < 4; j130 = j130 + 1) {
				fRec115_tmp[j130] = fRec115_perm[j130];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec115[i] = fSlow42 + fConst2 * fRec115[i - 1];
			}
			/* Post code */
			for (int j131 = 0; j131 < 4; j131 = j131 + 1) {
				fRec115_perm[j131] = fRec115_tmp[vsize + j131];
			}
			/* Recursive loop 130 */
			/* Pre code */
			for (int j242 = 0; j242 < 4; j242 = j242 + 1) {
				fRec178_tmp[j242] = fRec178_perm[j242];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec178[i] = std::max<float>(0.995f * fRec178[i - 1], std::fabs(fSlow40 * fZec251[i]));
			}
			/* Post code */
			for (int j243 = 0; j243 < 4; j243 = j243 + 1) {
				fRec178_perm[j243] = fRec178_tmp[vsize + j243];
			}
			/* Vectorizable loop 131 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec252[i] = fRec133[i] + fSlow39 * fRec132[i];
			}
			/* Recursive loop 132 */
			/* Pre code */
			for (int j124 = 0; j124 < 4; j124 = j124 + 1) {
				fRec68_tmp[j124] = fRec68_perm[j124];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec68[i] = fConst1 * static_cast<float>(fRec69[i] > fZec0[i]) + fConst2 * fRec68[i - 1];
			}
			/* Post code */
			for (int j125 = 0; j125 < 4; j125 = j125 + 1) {
				fRec68_perm[j125] = fRec68_tmp[vsize + j125];
			}
			/* Vectorizable loop 133 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec125[i] = mlczerov_faustpower2_f(fZec124[i]);
			}
			/* Vectorizable loop 134 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec130[i] = std::pow(1e+01f, fSlow44 * fRec115[i]);
			}
			/* Recursive loop 135 */
			/* Pre code */
			for (int j136 = 0; j136 < 4; j136 = j136 + 1) {
				fRec116_tmp[j136] = fRec116_perm[j136];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec116[i] = fSlow45 + fConst2 * fRec116[i - 1];
			}
			/* Post code */
			for (int j137 = 0; j137 < 4; j137 = j137 + 1) {
				fRec116_perm[j137] = fRec116_tmp[vsize + j137];
			}
			/* Recursive loop 136 */
			/* Pre code */
			for (int j244 = 0; j244 < 4; j244 = j244 + 1) {
				fRec177_tmp[j244] = fRec177_perm[j244];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec177[i] = fConst1 * static_cast<float>(fRec178[i] > fZec0[i]) + fConst2 * fRec177[i - 1];
			}
			/* Post code */
			for (int j245 = 0; j245 < 4; j245 = j245 + 1) {
				fRec177_perm[j245] = fRec177_tmp[vsize + j245];
			}
			/* Vectorizable loop 137 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec253[i] = mlczerov_faustpower2_f(fZec252[i]);
			}
			/* Recursive loop 138 */
			/* Pre code */
			for (int j126 = 0; j126 < 4; j126 = j126 + 1) {
				fRec7_tmp[j126] = fRec7_perm[j126];
			}
			for (int j128 = 0; j128 < 4; j128 = j128 + 1) {
				fRec8_tmp[j128] = fRec8_perm[j128];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec126[i] = ((iSlow41) ? fSlow40 * fRec68[i] * fZec123[i] : fSlow40 * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec125[i] + -1.0f) + fZec124[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec125[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec125[i] * (1.0f - fZec125[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec124[i])))) * ((fZec124[i] > 0.0f) ? 1.0f : ((fZec124[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec124[i]) - fSlow20 : tanhf(fZec124[i]))))) - (fConst65 * fRec7[i - 1] + fRec8[i - 1]);
				fRec7[i] = fRec7[i - 1] + fConst68 * fZec126[i];
				fZec127[i] = fRec7[i - 1] + fConst67 * fZec126[i];
				fRec8[i] = fRec8[i - 1] + fConst69 * fZec127[i];
				fZec128[i] = fConst64 * fZec127[i];
				fRec9[i] = fRec8[i - 1] + fZec128[i];
				fZec129[i] = fConst70 * fZec126[i];
				fRec10[i] = fZec129[i];
				fRec11[i] = fZec127[i];
			}
			/* Post code */
			for (int j127 = 0; j127 < 4; j127 = j127 + 1) {
				fRec7_perm[j127] = fRec7_tmp[vsize + j127];
			}
			for (int j129 = 0; j129 < 4; j129 = j129 + 1) {
				fRec8_perm[j129] = fRec8_tmp[vsize + j129];
			}
			/* Recursive loop 139 */
			/* Pre code */
			for (int j2 = 0; j2 < 4; j2 = j2 + 1) {
				fRec1_tmp[j2] = fRec1_perm[j2];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec1[i] = fSlow1 + fConst2 * fRec1[i - 1];
			}
			/* Post code */
			for (int j3 = 0; j3 < 4; j3 = j3 + 1) {
				fRec1_perm[j3] = fRec1_tmp[vsize + j3];
			}
			/* Vectorizable loop 140 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec131[i] = std::sqrt(fZec130[i]);
			}
			/* Vectorizable loop 141 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec139[i] = std::pow(1e+01f, fSlow46 * fRec116[i]);
			}
			/* Recursive loop 142 */
			/* Pre code */
			for (int j246 = 0; j246 < 4; j246 = j246 + 1) {
				fRec125_tmp[j246] = fRec125_perm[j246];
			}
			for (int j248 = 0; j248 < 4; j248 = j248 + 1) {
				fRec126_tmp[j248] = fRec126_perm[j248];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec254[i] = ((iSlow41) ? fSlow40 * fRec177[i] * fZec251[i] : fSlow40 * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec253[i] + -1.0f) + fZec252[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec253[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec253[i] * (1.0f - fZec253[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec252[i])))) * ((fZec252[i] > 0.0f) ? 1.0f : ((fZec252[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec252[i]) - fSlow20 : tanhf(fZec252[i]))))) - (fConst65 * fRec125[i - 1] + fRec126[i - 1]);
				fRec125[i] = fRec125[i - 1] + fConst68 * fZec254[i];
				fZec255[i] = fRec125[i - 1] + fConst67 * fZec254[i];
				fRec126[i] = fRec126[i - 1] + fConst69 * fZec255[i];
				fZec256[i] = fConst64 * fZec255[i];
				fRec127[i] = fRec126[i - 1] + fZec256[i];
				fZec257[i] = fConst70 * fZec254[i];
				fRec128[i] = fZec257[i];
				fRec129[i] = fZec255[i];
			}
			/* Post code */
			for (int j247 = 0; j247 < 4; j247 = j247 + 1) {
				fRec125_perm[j247] = fRec125_tmp[vsize + j247];
			}
			for (int j249 = 0; j249 < 4; j249 = j249 + 1) {
				fRec126_perm[j249] = fRec126_tmp[vsize + j249];
			}
			/* Recursive loop 143 */
			/* Pre code */
			for (int j132 = 0; j132 < 4; j132 = j132 + 1) {
				fRec2_tmp[j132] = fRec2_perm[j132];
			}
			for (int j134 = 0; j134 < 4; j134 = j134 + 1) {
				fRec3_tmp[j134] = fRec3_perm[j134];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec132[i] = fRec10[i] + fRec9[i] * fZec130[i] + 1.25f * fRec11[i] * fZec131[i] - (fConst72 * fRec2[i - 1] + fRec3[i - 1]);
				fRec2[i] = fRec2[i - 1] + fConst75 * fZec132[i];
				fZec133[i] = fRec2[i - 1] + fConst74 * fZec132[i];
				fRec3[i] = fRec3[i - 1] + fConst76 * fZec133[i];
				fZec134[i] = fConst71 * fZec133[i];
				fRec4[i] = fRec3[i - 1] + fZec134[i];
				fZec135[i] = fConst77 * fZec132[i];
				fRec5[i] = fZec135[i];
				fRec6[i] = fZec133[i];
			}
			/* Post code */
			for (int j133 = 0; j133 < 4; j133 = j133 + 1) {
				fRec2_perm[j133] = fRec2_tmp[vsize + j133];
			}
			for (int j135 = 0; j135 < 4; j135 = j135 + 1) {
				fRec3_perm[j135] = fRec3_tmp[vsize + j135];
			}
			/* Recursive loop 144 */
			/* Pre code */
			for (int j0 = 0; j0 < 4; j0 = j0 + 1) {
				fRec0_tmp[j0] = fRec0_perm[j0];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec0[i] = fSlow0 + fConst2 * fRec0[i - 1];
			}
			/* Post code */
			for (int j1 = 0; j1 < 4; j1 = j1 + 1) {
				fRec0_perm[j1] = fRec0_tmp[vsize + j1];
			}
			/* Recursive loop 145 */
			/* Pre code */
			for (int j138 = 0; j138 < 4; j138 = j138 + 1) {
				fRec117_tmp[j138] = fRec117_perm[j138];
			}
			for (int j140 = 0; j140 < 4; j140 = j140 + 1) {
				fRec118_tmp[j140] = fRec118_perm[j140];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec136[i] = static_cast<float>(input0[i]) - (fConst79 * fRec117[i - 1] + fRec118[i - 1]);
				fRec117[i] = fRec117[i - 1] + fConst82 * fZec136[i];
				fZec137[i] = fRec117[i - 1] + fConst81 * fZec136[i];
				fRec118[i] = fRec118[i - 1] + fConst83 * fZec137[i];
				fZec138[i] = fConst78 * fZec137[i];
				fRec119[i] = fRec118[i - 1] + fZec138[i];
			}
			/* Post code */
			for (int j139 = 0; j139 < 4; j139 = j139 + 1) {
				fRec117_perm[j139] = fRec117_tmp[vsize + j139];
			}
			for (int j141 = 0; j141 < 4; j141 = j141 + 1) {
				fRec118_perm[j141] = fRec118_tmp[vsize + j141];
			}
			/* Vectorizable loop 146 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec140[i] = std::sqrt(fZec139[i]);
			}
			/* Vectorizable loop 147 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec141[i] = 1.0f - fRec1[i];
			}
			/* Recursive loop 148 */
			/* Pre code */
			for (int j250 = 0; j250 < 4; j250 = j250 + 1) {
				fRec120_tmp[j250] = fRec120_perm[j250];
			}
			for (int j252 = 0; j252 < 4; j252 = j252 + 1) {
				fRec121_tmp[j252] = fRec121_perm[j252];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec258[i] = fRec128[i] + fRec127[i] * fZec130[i] + 1.25f * fRec129[i] * fZec131[i] - (fConst72 * fRec120[i - 1] + fRec121[i - 1]);
				fRec120[i] = fRec120[i - 1] + fConst75 * fZec258[i];
				fZec259[i] = fRec120[i - 1] + fConst74 * fZec258[i];
				fRec121[i] = fRec121[i - 1] + fConst76 * fZec259[i];
				fZec260[i] = fConst71 * fZec259[i];
				fRec122[i] = fRec121[i - 1] + fZec260[i];
				fZec261[i] = fConst77 * fZec258[i];
				fRec123[i] = fZec261[i];
				fRec124[i] = fZec259[i];
			}
			/* Post code */
			for (int j251 = 0; j251 < 4; j251 = j251 + 1) {
				fRec120_perm[j251] = fRec120_tmp[vsize + j251];
			}
			for (int j253 = 0; j253 < 4; j253 = j253 + 1) {
				fRec121_perm[j253] = fRec121_tmp[vsize + j253];
			}
			/* Recursive loop 149 */
			/* Pre code */
			for (int j254 = 0; j254 < 4; j254 = j254 + 1) {
				fRec224_tmp[j254] = fRec224_perm[j254];
			}
			for (int j256 = 0; j256 < 4; j256 = j256 + 1) {
				fRec225_tmp[j256] = fRec225_perm[j256];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec262[i] = static_cast<float>(input1[i]) - (fConst79 * fRec224[i - 1] + fRec225[i - 1]);
				fRec224[i] = fRec224[i - 1] + fConst82 * fZec262[i];
				fZec263[i] = fRec224[i - 1] + fConst81 * fZec262[i];
				fRec225[i] = fRec225[i - 1] + fConst83 * fZec263[i];
				fZec264[i] = fConst78 * fZec263[i];
				fRec226[i] = fRec225[i - 1] + fZec264[i];
			}
			/* Post code */
			for (int j255 = 0; j255 < 4; j255 = j255 + 1) {
				fRec224_perm[j255] = fRec224_tmp[vsize + j255];
			}
			for (int j257 = 0; j257 < 4; j257 = j257 + 1) {
				fRec225_perm[j257] = fRec225_tmp[vsize + j257];
			}
			/* Vectorizable loop 150 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fZec141[i] * (fRec4[i] + fRec5[i] * fZec139[i] + 1.4285715f * fRec6[i] * fZec140[i]) + fRec1[i] * fRec119[i]));
			}
			/* Vectorizable loop 151 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fZec141[i] * (fRec122[i] + fRec123[i] * fZec139[i] + 1.4285715f * fRec124[i] * fZec140[i]) + fRec1[i] * fRec226[i]));
			}
		}
		/* Remaining frames */
		if (vindex < count) {
			FAUSTFLOAT* input0 = &input0_ptr[vindex];
			FAUSTFLOAT* input1 = &input1_ptr[vindex];
			FAUSTFLOAT* output0 = &output0_ptr[vindex];
			FAUSTFLOAT* output1 = &output1_ptr[vindex];
			int vsize = count - vindex;
			/* Recursive loop 0 */
			/* Pre code */
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec54_tmp[j10] = fRec54_perm[j10];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec54[i] = fSlow3 + fConst2 * fRec54[i - 1];
			}
			/* Post code */
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec54_perm[j11] = fRec54_tmp[vsize + j11];
			}
			/* Vectorizable loop 1 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec142[i] = static_cast<float>(input1[i]) * fRec54[i];
			}
			/* Recursive loop 2 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec53_tmp[j6] = fRec53_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec53[i] = fSlow2 + fConst2 * fRec53[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec53_perm[j7] = fRec53_tmp[vsize + j7];
			}
			/* Recursive loop 3 */
			/* Pre code */
			for (int j74 = 0; j74 < 4; j74 = j74 + 1) {
				fRec104_tmp[j74] = fRec104_perm[j74];
			}
			for (int j76 = 0; j76 < 4; j76 = j76 + 1) {
				fRec105_tmp[j76] = fRec105_perm[j76];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec66[i] = static_cast<float>(input0[i]) * fRec54[i] - (fConst4 * fRec104[i - 1] + fRec105[i - 1]);
				fRec104[i] = fRec104[i - 1] + fConst7 * fZec66[i];
				fZec67[i] = fRec104[i - 1] + fConst6 * fZec66[i];
				fRec105[i] = fRec105[i - 1] + fConst8 * fZec67[i];
				fZec68[i] = fConst3 * fZec67[i];
				fRec106[i] = fRec105[i - 1] + fZec68[i];
				fZec69[i] = fConst9 * fZec66[i];
				fRec107[i] = fZec69[i];
				fRec108[i] = fZec67[i];
			}
			/* Post code */
			for (int j75 = 0; j75 < 4; j75 = j75 + 1) {
				fRec104_perm[j75] = fRec104_tmp[vsize + j75];
			}
			for (int j77 = 0; j77 < 4; j77 = j77 + 1) {
				fRec105_perm[j77] = fRec105_tmp[vsize + j77];
			}
			/* Recursive loop 4 */
			/* Pre code */
			for (int j194 = 0; j194 < 4; j194 = j194 + 1) {
				fRec213_tmp[j194] = fRec213_perm[j194];
			}
			for (int j196 = 0; j196 < 4; j196 = j196 + 1) {
				fRec214_tmp[j196] = fRec214_perm[j196];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec196[i] = fZec142[i] - (fConst4 * fRec213[i - 1] + fRec214[i - 1]);
				fRec213[i] = fRec213[i - 1] + fConst7 * fZec196[i];
				fZec197[i] = fRec213[i - 1] + fConst6 * fZec196[i];
				fRec214[i] = fRec214[i - 1] + fConst8 * fZec197[i];
				fZec198[i] = fConst3 * fZec197[i];
				fRec215[i] = fRec214[i - 1] + fZec198[i];
				fZec199[i] = fConst9 * fZec196[i];
				fRec216[i] = fZec199[i];
				fRec217[i] = fZec197[i];
			}
			/* Post code */
			for (int j195 = 0; j195 < 4; j195 = j195 + 1) {
				fRec213_perm[j195] = fRec213_tmp[vsize + j195];
			}
			for (int j197 = 0; j197 < 4; j197 = j197 + 1) {
				fRec214_perm[j197] = fRec214_tmp[vsize + j197];
			}
			/* Recursive loop 5 */
			/* Pre code */
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec52_tmp[j4] = fRec52_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec52[i] = std::max<float>(0.995f * fRec52[i - 1], std::fabs(static_cast<float>(input0[i])));
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec52_perm[j5] = fRec52_tmp[vsize + j5];
			}
			/* Vectorizable loop 6 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::pow(1e+01f, 0.05f * fRec53[i]);
			}
			/* Vectorizable loop 7 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = 1.0f - 0.7f * fRec54[i];
			}
			/* Vectorizable loop 8 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = 72.0f * fRec54[i] + 8.0f;
			}
			/* Recursive loop 9 */
			/* Pre code */
			for (int j78 = 0; j78 < 4; j78 = j78 + 1) {
				fRec100_tmp[j78] = fRec100_perm[j78];
			}
			for (int j80 = 0; j80 < 4; j80 = j80 + 1) {
				fRec101_tmp[j80] = fRec101_perm[j80];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec70[i] = fRec107[i] + fSlow5 * fRec106[i] + fSlow6 * fRec108[i] - (fConst11 * fRec100[i - 1] + fRec101[i - 1]);
				fRec100[i] = fRec100[i - 1] + fConst14 * fZec70[i];
				fZec71[i] = fRec100[i - 1] + fConst13 * fZec70[i];
				fRec101[i] = fRec101[i - 1] + fConst15 * fZec71[i];
				fRec102[i] = fZec71[i];
				fZec72[i] = fConst16 * fZec70[i];
				fZec73[i] = fConst10 * fZec71[i];
				fRec103[i] = fZec73[i] + fRec101[i - 1] + fZec72[i];
			}
			/* Post code */
			for (int j79 = 0; j79 < 4; j79 = j79 + 1) {
				fRec100_perm[j79] = fRec100_tmp[vsize + j79];
			}
			for (int j81 = 0; j81 < 4; j81 = j81 + 1) {
				fRec101_perm[j81] = fRec101_tmp[vsize + j81];
			}
			/* Recursive loop 10 */
			/* Pre code */
			for (int j142 = 0; j142 < 4; j142 = j142 + 1) {
				fRec170_tmp[j142] = fRec170_perm[j142];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec170[i] = std::max<float>(0.995f * fRec170[i - 1], std::fabs(static_cast<float>(input1[i])));
			}
			/* Post code */
			for (int j143 = 0; j143 < 4; j143 = j143 + 1) {
				fRec170_perm[j143] = fRec170_tmp[vsize + j143];
			}
			/* Recursive loop 11 */
			/* Pre code */
			for (int j198 = 0; j198 < 4; j198 = j198 + 1) {
				fRec209_tmp[j198] = fRec209_perm[j198];
			}
			for (int j200 = 0; j200 < 4; j200 = j200 + 1) {
				fRec210_tmp[j200] = fRec210_perm[j200];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec200[i] = fRec216[i] + fSlow5 * fRec215[i] + fSlow6 * fRec217[i] - (fConst11 * fRec209[i - 1] + fRec210[i - 1]);
				fRec209[i] = fRec209[i - 1] + fConst14 * fZec200[i];
				fZec201[i] = fRec209[i - 1] + fConst13 * fZec200[i];
				fRec210[i] = fRec210[i - 1] + fConst15 * fZec201[i];
				fRec211[i] = fZec201[i];
				fZec202[i] = fConst16 * fZec200[i];
				fZec203[i] = fConst10 * fZec201[i];
				fRec212[i] = fZec203[i] + fRec210[i - 1] + fZec202[i];
			}
			/* Post code */
			for (int j199 = 0; j199 < 4; j199 = j199 + 1) {
				fRec209_perm[j199] = fRec209_tmp[vsize + j199];
			}
			for (int j201 = 0; j201 < 4; j201 = j201 + 1) {
				fRec210_perm[j201] = fRec210_tmp[vsize + j201];
			}
			/* Recursive loop 12 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec51_tmp[j8] = fRec51_perm[j8];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec51[i] = fConst1 * static_cast<float>(fRec52[i] > fZec0[i]) + fConst2 * fRec51[i - 1];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec51_perm[j9] = fRec51_tmp[vsize + j9];
			}
			/* Recursive loop 13 */
			/* Pre code */
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec55_tmp[j20] = fRec55_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec55[i] = fSlow7 + fConst2 * fRec55[i - 1];
			}
			/* Post code */
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec55_perm[j21] = fRec55_tmp[vsize + j21];
			}
			/* Recursive loop 14 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec56_tmp[j22] = fRec56_perm[j22];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec56[i] = fSlow8 + fConst2 * fRec56[i - 1];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec56_perm[j23] = fRec56_tmp[vsize + j23];
			}
			/* Recursive loop 15 */
			/* Pre code */
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec57_tmp[j24] = fRec57_perm[j24];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec57[i] = fSlow9 + fConst2 * fRec57[i - 1];
			}
			/* Post code */
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec57_perm[j25] = fRec57_tmp[vsize + j25];
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec14[i] = mlczerov_faustpower2_f(fZec9[i]);
			}
			/* Vectorizable loop 17 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec15[i] = mlczerov_faustpower2_f(fZec10[i]);
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec74[i] = fRec103[i] + fSlow14 * fRec102[i];
			}
			/* Vectorizable loop 19 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec75[i] = fZec10[i] * fZec9[i];
			}
			/* Recursive loop 20 */
			/* Pre code */
			for (int j144 = 0; j144 < 4; j144 = j144 + 1) {
				fRec169_tmp[j144] = fRec169_perm[j144];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec169[i] = fConst1 * static_cast<float>(fRec170[i] > fZec0[i]) + fConst2 * fRec169[i - 1];
			}
			/* Post code */
			for (int j145 = 0; j145 < 4; j145 = j145 + 1) {
				fRec169_perm[j145] = fRec169_tmp[vsize + j145];
			}
			/* Vectorizable loop 21 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec204[i] = fRec212[i] + fSlow14 * fRec211[i];
			}
			/* Recursive loop 22 */
			/* Pre code */
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec46_tmp[j12] = fRec46_perm[j12];
			}
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec47_tmp[j14] = fRec47_perm[j14];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec1[i] = static_cast<float>(input0[i]) * fRec51[i] * fRec54[i] - (fConst4 * fRec46[i - 1] + fRec47[i - 1]);
				fRec46[i] = fRec46[i - 1] + fConst7 * fZec1[i];
				fZec2[i] = fRec46[i - 1] + fConst6 * fZec1[i];
				fRec47[i] = fRec47[i - 1] + fConst8 * fZec2[i];
				fZec3[i] = fConst3 * fZec2[i];
				fRec48[i] = fRec47[i - 1] + fZec3[i];
				fZec4[i] = fConst9 * fZec1[i];
				fRec49[i] = fZec4[i];
				fRec50[i] = fZec2[i];
			}
			/* Post code */
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec46_perm[j13] = fRec46_tmp[vsize + j13];
			}
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec47_perm[j15] = fRec47_tmp[vsize + j15];
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec17[i] = 1.0f - (fRec56[i] + fRec55[i] + fRec57[i]);
			}
			/* Vectorizable loop 24 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec76[i] = fZec75[i] * fZec74[i];
			}
			/* Vectorizable loop 25 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec78[i] = fZec15[i] * fZec14[i];
			}
			/* Recursive loop 26 */
			/* Pre code */
			for (int j146 = 0; j146 < 4; j146 = j146 + 1) {
				fRec164_tmp[j146] = fRec164_perm[j146];
			}
			for (int j148 = 0; j148 < 4; j148 = j148 + 1) {
				fRec165_tmp[j148] = fRec165_perm[j148];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec143[i] = fZec142[i] * fRec169[i] - (fConst4 * fRec164[i - 1] + fRec165[i - 1]);
				fRec164[i] = fRec164[i - 1] + fConst7 * fZec143[i];
				fZec144[i] = fRec164[i - 1] + fConst6 * fZec143[i];
				fRec165[i] = fRec165[i - 1] + fConst8 * fZec144[i];
				fZec145[i] = fConst3 * fZec144[i];
				fRec166[i] = fRec165[i - 1] + fZec145[i];
				fZec146[i] = fConst9 * fZec143[i];
				fRec167[i] = fZec146[i];
				fRec168[i] = fZec144[i];
			}
			/* Post code */
			for (int j147 = 0; j147 < 4; j147 = j147 + 1) {
				fRec164_perm[j147] = fRec164_tmp[vsize + j147];
			}
			for (int j149 = 0; j149 < 4; j149 = j149 + 1) {
				fRec165_perm[j149] = fRec165_tmp[vsize + j149];
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec205[i] = fZec75[i] * fZec204[i];
			}
			/* Recursive loop 28 */
			/* Pre code */
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec42_tmp[j16] = fRec42_perm[j16];
			}
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec43_tmp[j18] = fRec43_perm[j18];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec5[i] = fRec49[i] + fSlow5 * fRec48[i] + fSlow6 * fRec50[i] - (fConst11 * fRec42[i - 1] + fRec43[i - 1]);
				fRec42[i] = fRec42[i - 1] + fConst14 * fZec5[i];
				fZec6[i] = fRec42[i - 1] + fConst13 * fZec5[i];
				fRec43[i] = fRec43[i - 1] + fConst15 * fZec6[i];
				fRec44[i] = fZec6[i];
				fZec7[i] = fConst16 * fZec5[i];
				fZec8[i] = fConst10 * fZec6[i];
				fRec45[i] = fZec8[i] + fRec43[i - 1] + fZec7[i];
			}
			/* Post code */
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec42_perm[j17] = fRec42_tmp[vsize + j17];
			}
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec43_perm[j19] = fRec43_tmp[vsize + j19];
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec18[i] = 0.22f * fZec17[i];
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec77[i] = fSlow18 * fZec76[i];
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec79[i] = fZec78[i] * mlczerov_faustpower2_f(fZec74[i]);
			}
			/* Recursive loop 32 */
			/* Pre code */
			for (int j150 = 0; j150 < 4; j150 = j150 + 1) {
				fRec160_tmp[j150] = fRec160_perm[j150];
			}
			for (int j152 = 0; j152 < 4; j152 = j152 + 1) {
				fRec161_tmp[j152] = fRec161_perm[j152];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec147[i] = fRec167[i] + fSlow5 * fRec166[i] + fSlow6 * fRec168[i] - (fConst11 * fRec160[i - 1] + fRec161[i - 1]);
				fRec160[i] = fRec160[i - 1] + fConst14 * fZec147[i];
				fZec148[i] = fRec160[i - 1] + fConst13 * fZec147[i];
				fRec161[i] = fRec161[i - 1] + fConst15 * fZec148[i];
				fRec162[i] = fZec148[i];
				fZec149[i] = fConst16 * fZec147[i];
				fZec150[i] = fConst10 * fZec148[i];
				fRec163[i] = fZec150[i] + fRec161[i - 1] + fZec149[i];
			}
			/* Post code */
			for (int j151 = 0; j151 < 4; j151 = j151 + 1) {
				fRec160_perm[j151] = fRec160_tmp[vsize + j151];
			}
			for (int j153 = 0; j153 < 4; j153 = j153 + 1) {
				fRec161_perm[j153] = fRec161_tmp[vsize + j153];
			}
			/* Vectorizable loop 33 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec206[i] = fSlow18 * fZec205[i];
			}
			/* Vectorizable loop 34 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec207[i] = fZec78[i] * mlczerov_faustpower2_f(fZec204[i]);
			}
			/* Vectorizable loop 35 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = fRec45[i] + fSlow14 * fRec44[i];
			}
			/* Vectorizable loop 36 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec80[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec79[i] + -1.0f) + fSlow17 * fZec76[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec79[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec79[i] * (fSlow22 * fZec79[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec77[i])))) * ((fZec77[i] > 0.0f) ? 1.0f : ((fZec77[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec77[i]) - fSlow20 : tanhf(fZec77[i]))));
			}
			/* Vectorizable loop 37 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec151[i] = fRec163[i] + fSlow14 * fRec162[i];
			}
			/* Vectorizable loop 38 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec208[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec207[i] + -1.0f) + fSlow17 * fZec205[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec207[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec207[i] * (fSlow22 * fZec207[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec206[i])))) * ((fZec206[i] > 0.0f) ? 1.0f : ((fZec206[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec206[i]) - fSlow20 : tanhf(fZec206[i]))));
			}
			/* Vectorizable loop 39 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = fZec11[i] * fZec10[i] * fZec9[i];
			}
			/* Recursive loop 40 */
			/* Pre code */
			for (int j82 = 0; j82 < 4; j82 = j82 + 1) {
				fRec109_tmp[j82] = fRec109_perm[j82];
			}
			for (int j84 = 0; j84 < 4; j84 = j84 + 1) {
				fRec110_tmp[j84] = fRec110_perm[j84];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec81[i] = fZec80[i] - (fConst18 * fRec109[i - 1] + fRec110[i - 1]);
				fRec109[i] = fRec109[i - 1] + fConst21 * fZec81[i];
				fZec82[i] = fRec109[i - 1] + fConst20 * fZec81[i];
				fRec110[i] = fRec110[i - 1] + fConst22 * fZec82[i];
				fZec83[i] = fConst23 * fZec81[i];
				fRec111[i] = fZec83[i];
			}
			/* Post code */
			for (int j83 = 0; j83 < 4; j83 = j83 + 1) {
				fRec109_perm[j83] = fRec109_tmp[vsize + j83];
			}
			for (int j85 = 0; j85 < 4; j85 = j85 + 1) {
				fRec110_perm[j85] = fRec110_tmp[vsize + j85];
			}
			/* Vectorizable loop 41 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec152[i] = fZec75[i] * fZec151[i];
			}
			/* Recursive loop 42 */
			/* Pre code */
			for (int j202 = 0; j202 < 4; j202 = j202 + 1) {
				fRec218_tmp[j202] = fRec218_perm[j202];
			}
			for (int j204 = 0; j204 < 4; j204 = j204 + 1) {
				fRec219_tmp[j204] = fRec219_perm[j204];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec209[i] = fZec208[i] - (fConst18 * fRec218[i - 1] + fRec219[i - 1]);
				fRec218[i] = fRec218[i - 1] + fConst21 * fZec209[i];
				fZec210[i] = fRec218[i - 1] + fConst20 * fZec209[i];
				fRec219[i] = fRec219[i - 1] + fConst22 * fZec210[i];
				fZec211[i] = fConst23 * fZec209[i];
				fRec220[i] = fZec211[i];
			}
			/* Post code */
			for (int j203 = 0; j203 < 4; j203 = j203 + 1) {
				fRec218_perm[j203] = fRec218_tmp[vsize + j203];
			}
			for (int j205 = 0; j205 < 4; j205 = j205 + 1) {
				fRec219_perm[j205] = fRec219_tmp[vsize + j205];
			}
			/* Vectorizable loop 43 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec13[i] = fSlow18 * fZec12[i];
			}
			/* Vectorizable loop 44 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec16[i] = mlczerov_faustpower2_f(fZec11[i]) * fZec15[i] * fZec14[i];
			}
			/* Recursive loop 45 */
			/* Pre code */
			for (int j86 = 0; j86 < 4; j86 = j86 + 1) {
				fRec97_tmp[j86] = fRec97_perm[j86];
			}
			for (int j88 = 0; j88 < 4; j88 = j88 + 1) {
				fRec98_tmp[j88] = fRec98_perm[j88];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec84[i] = ((iSlow25) ? fRec111[i] : fZec80[i]) - (fConst18 * fRec97[i - 1] + fRec98[i - 1]);
				fRec97[i] = fRec97[i - 1] + fConst21 * fZec84[i];
				fZec85[i] = fRec97[i - 1] + fConst20 * fZec84[i];
				fRec98[i] = fRec98[i - 1] + fConst22 * fZec85[i];
				fZec86[i] = fConst23 * fZec84[i];
				fRec99[i] = fZec86[i];
			}
			/* Post code */
			for (int j87 = 0; j87 < 4; j87 = j87 + 1) {
				fRec97_perm[j87] = fRec97_tmp[vsize + j87];
			}
			for (int j89 = 0; j89 < 4; j89 = j89 + 1) {
				fRec98_perm[j89] = fRec98_tmp[vsize + j89];
			}
			/* Vectorizable loop 46 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec153[i] = fSlow18 * fZec152[i];
			}
			/* Vectorizable loop 47 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec154[i] = fZec78[i] * mlczerov_faustpower2_f(fZec151[i]);
			}
			/* Recursive loop 48 */
			/* Pre code */
			for (int j206 = 0; j206 < 4; j206 = j206 + 1) {
				fRec206_tmp[j206] = fRec206_perm[j206];
			}
			for (int j208 = 0; j208 < 4; j208 = j208 + 1) {
				fRec207_tmp[j208] = fRec207_perm[j208];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec212[i] = ((iSlow25) ? fRec220[i] : fZec208[i]) - (fConst18 * fRec206[i - 1] + fRec207[i - 1]);
				fRec206[i] = fRec206[i - 1] + fConst21 * fZec212[i];
				fZec213[i] = fRec206[i - 1] + fConst20 * fZec212[i];
				fRec207[i] = fRec207[i - 1] + fConst22 * fZec213[i];
				fZec214[i] = fConst23 * fZec212[i];
				fRec208[i] = fZec214[i];
			}
			/* Post code */
			for (int j207 = 0; j207 < 4; j207 = j207 + 1) {
				fRec206_perm[j207] = fRec206_tmp[vsize + j207];
			}
			for (int j209 = 0; j209 < 4; j209 = j209 + 1) {
				fRec207_perm[j209] = fRec207_tmp[vsize + j209];
			}
			/* Vectorizable loop 49 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec19[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec16[i] + -1.0f) + fSlow17 * fZec12[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec16[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec16[i] * (fSlow22 * fZec16[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec13[i])))) * ((fZec13[i] > 0.0f) ? 1.0f : ((fZec13[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec13[i]) - fSlow20 : tanhf(fZec13[i]))));
			}
			/* Recursive loop 50 */
			/* Pre code */
			for (int j90 = 0; j90 < 4; j90 = j90 + 1) {
				fRec94_tmp[j90] = fRec94_perm[j90];
			}
			for (int j92 = 0; j92 < 4; j92 = j92 + 1) {
				fRec95_tmp[j92] = fRec95_perm[j92];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec87[i] = fRec99[i] - (fConst25 * fRec94[i - 1] + fRec95[i - 1]);
				fRec94[i] = fRec94[i - 1] + fConst28 * fZec87[i];
				fZec88[i] = fRec94[i - 1] + fConst27 * fZec87[i];
				fRec95[i] = fRec95[i - 1] + fConst29 * fZec88[i];
				fZec89[i] = fConst24 * fZec88[i];
				fRec96[i] = fRec95[i - 1] + fZec89[i];
			}
			/* Post code */
			for (int j91 = 0; j91 < 4; j91 = j91 + 1) {
				fRec94_perm[j91] = fRec94_tmp[vsize + j91];
			}
			for (int j93 = 0; j93 < 4; j93 = j93 + 1) {
				fRec95_perm[j93] = fRec95_tmp[vsize + j93];
			}
			/* Vectorizable loop 51 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec155[i] = 0.78f * ((iSlow11) ? tanhf(0.5f * fRec55[i] * (fSlow24 * fZec154[i] + -1.0f) + fSlow17 * fZec152[i] * (fZec18[i] + 0.33f * fRec57[i] * (fSlow23 * fZec154[i] + -0.66f)) + 0.25f * fRec56[i] * (fSlow21 * fZec154[i] * (fSlow22 * fZec154[i] + -0.3872f) + 1.0f)) : ((iSlow12) ? (1.0f - std::exp(-(std::fabs(fZec153[i])))) * ((fZec153[i] > 0.0f) ? 1.0f : ((fZec153[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec153[i]) - fSlow20 : tanhf(fZec153[i]))));
			}
			/* Recursive loop 52 */
			/* Pre code */
			for (int j210 = 0; j210 < 4; j210 = j210 + 1) {
				fRec203_tmp[j210] = fRec203_perm[j210];
			}
			for (int j212 = 0; j212 < 4; j212 = j212 + 1) {
				fRec204_tmp[j212] = fRec204_perm[j212];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec215[i] = fRec208[i] - (fConst25 * fRec203[i - 1] + fRec204[i - 1]);
				fRec203[i] = fRec203[i - 1] + fConst28 * fZec215[i];
				fZec216[i] = fRec203[i - 1] + fConst27 * fZec215[i];
				fRec204[i] = fRec204[i - 1] + fConst29 * fZec216[i];
				fZec217[i] = fConst24 * fZec216[i];
				fRec205[i] = fRec204[i - 1] + fZec217[i];
			}
			/* Post code */
			for (int j211 = 0; j211 < 4; j211 = j211 + 1) {
				fRec203_perm[j211] = fRec203_tmp[vsize + j211];
			}
			for (int j213 = 0; j213 < 4; j213 = j213 + 1) {
				fRec204_perm[j213] = fRec204_tmp[vsize + j213];
			}
			/* Recursive loop 53 */
			/* Pre code */
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec58_tmp[j26] = fRec58_perm[j26];
			}
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec59_tmp[j28] = fRec59_perm[j28];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec20[i] = fZec19[i] - (fConst18 * fRec58[i - 1] + fRec59[i - 1]);
				fRec58[i] = fRec58[i - 1] + fConst21 * fZec20[i];
				fZec21[i] = fRec58[i - 1] + fConst20 * fZec20[i];
				fRec59[i] = fRec59[i - 1] + fConst22 * fZec21[i];
				fZec22[i] = fConst23 * fZec20[i];
				fRec60[i] = fZec22[i];
			}
			/* Post code */
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec58_perm[j27] = fRec58_tmp[vsize + j27];
			}
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec59_perm[j29] = fRec59_tmp[vsize + j29];
			}
			/* Vectorizable loop 54 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec90[i] = fSlow29 * fRec96[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 55 */
			/* Pre code */
			for (int j154 = 0; j154 < 4; j154 = j154 + 1) {
				fRec171_tmp[j154] = fRec171_perm[j154];
			}
			for (int j156 = 0; j156 < 4; j156 = j156 + 1) {
				fRec172_tmp[j156] = fRec172_perm[j156];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec156[i] = fZec155[i] - (fConst18 * fRec171[i - 1] + fRec172[i - 1]);
				fRec171[i] = fRec171[i - 1] + fConst21 * fZec156[i];
				fZec157[i] = fRec171[i - 1] + fConst20 * fZec156[i];
				fRec172[i] = fRec172[i - 1] + fConst22 * fZec157[i];
				fZec158[i] = fConst23 * fZec156[i];
				fRec173[i] = fZec158[i];
			}
			/* Post code */
			for (int j155 = 0; j155 < 4; j155 = j155 + 1) {
				fRec171_perm[j155] = fRec171_tmp[vsize + j155];
			}
			for (int j157 = 0; j157 < 4; j157 = j157 + 1) {
				fRec172_perm[j157] = fRec172_tmp[vsize + j157];
			}
			/* Vectorizable loop 56 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec218[i] = fSlow29 * fRec205[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 57 */
			/* Pre code */
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec39_tmp[j30] = fRec39_perm[j30];
			}
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec40_tmp[j32] = fRec40_perm[j32];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec23[i] = ((iSlow25) ? fRec60[i] : fZec19[i]) - (fConst18 * fRec39[i - 1] + fRec40[i - 1]);
				fRec39[i] = fRec39[i - 1] + fConst21 * fZec23[i];
				fZec24[i] = fRec39[i - 1] + fConst20 * fZec23[i];
				fRec40[i] = fRec40[i - 1] + fConst22 * fZec24[i];
				fZec25[i] = fConst23 * fZec23[i];
				fRec41[i] = fZec25[i];
			}
			/* Post code */
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec39_perm[j31] = fRec39_tmp[vsize + j31];
			}
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec40_perm[j33] = fRec40_tmp[vsize + j33];
			}
			/* Vectorizable loop 58 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec91[i] = mlczerov_faustpower2_f(fZec90[i]);
			}
			/* Recursive loop 59 */
			/* Pre code */
			for (int j158 = 0; j158 < 4; j158 = j158 + 1) {
				fRec157_tmp[j158] = fRec157_perm[j158];
			}
			for (int j160 = 0; j160 < 4; j160 = j160 + 1) {
				fRec158_tmp[j160] = fRec158_perm[j160];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec159[i] = ((iSlow25) ? fRec173[i] : fZec155[i]) - (fConst18 * fRec157[i - 1] + fRec158[i - 1]);
				fRec157[i] = fRec157[i - 1] + fConst21 * fZec159[i];
				fZec160[i] = fRec157[i - 1] + fConst20 * fZec159[i];
				fRec158[i] = fRec158[i - 1] + fConst22 * fZec160[i];
				fZec161[i] = fConst23 * fZec159[i];
				fRec159[i] = fZec161[i];
			}
			/* Post code */
			for (int j159 = 0; j159 < 4; j159 = j159 + 1) {
				fRec157_perm[j159] = fRec157_tmp[vsize + j159];
			}
			for (int j161 = 0; j161 < 4; j161 = j161 + 1) {
				fRec158_perm[j161] = fRec158_tmp[vsize + j161];
			}
			/* Vectorizable loop 60 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec219[i] = mlczerov_faustpower2_f(fZec218[i]);
			}
			/* Recursive loop 61 */
			/* Pre code */
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec36_tmp[j34] = fRec36_perm[j34];
			}
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec37_tmp[j36] = fRec37_perm[j36];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec26[i] = fRec41[i] - (fConst25 * fRec36[i - 1] + fRec37[i - 1]);
				fRec36[i] = fRec36[i - 1] + fConst28 * fZec26[i];
				fZec27[i] = fRec36[i - 1] + fConst27 * fZec26[i];
				fRec37[i] = fRec37[i - 1] + fConst29 * fZec27[i];
				fZec28[i] = fConst24 * fZec27[i];
				fRec38[i] = fRec37[i - 1] + fZec28[i];
			}
			/* Post code */
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec36_perm[j35] = fRec36_tmp[vsize + j35];
			}
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec37_perm[j37] = fRec37_tmp[vsize + j37];
			}
			/* Recursive loop 62 */
			/* Pre code */
			for (int j94 = 0; j94 < 4; j94 = j94 + 1) {
				fRec91_tmp[j94] = fRec91_perm[j94];
			}
			for (int j96 = 0; j96 < 4; j96 = j96 + 1) {
				fRec92_tmp[j96] = fRec92_perm[j96];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec92[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec91[i] + -1.0f) + fZec90[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec91[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec91[i] * (1.0f - fZec91[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec90[i])))) * ((fZec90[i] > 0.0f) ? 1.0f : ((fZec90[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec90[i]) - fSlow20 : tanhf(fZec90[i])))) - (fConst18 * fRec91[i - 1] + fRec92[i - 1]);
				fRec91[i] = fRec91[i - 1] + fConst21 * fZec92[i];
				fZec93[i] = fRec91[i - 1] + fConst20 * fZec92[i];
				fRec92[i] = fRec92[i - 1] + fConst22 * fZec93[i];
				fZec94[i] = fConst23 * fZec92[i];
				fRec93[i] = fZec94[i];
			}
			/* Post code */
			for (int j95 = 0; j95 < 4; j95 = j95 + 1) {
				fRec91_perm[j95] = fRec91_tmp[vsize + j95];
			}
			for (int j97 = 0; j97 < 4; j97 = j97 + 1) {
				fRec92_perm[j97] = fRec92_tmp[vsize + j97];
			}
			/* Recursive loop 63 */
			/* Pre code */
			for (int j162 = 0; j162 < 4; j162 = j162 + 1) {
				fRec154_tmp[j162] = fRec154_perm[j162];
			}
			for (int j164 = 0; j164 < 4; j164 = j164 + 1) {
				fRec155_tmp[j164] = fRec155_perm[j164];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec162[i] = fRec159[i] - (fConst25 * fRec154[i - 1] + fRec155[i - 1]);
				fRec154[i] = fRec154[i - 1] + fConst28 * fZec162[i];
				fZec163[i] = fRec154[i - 1] + fConst27 * fZec162[i];
				fRec155[i] = fRec155[i - 1] + fConst29 * fZec163[i];
				fZec164[i] = fConst24 * fZec163[i];
				fRec156[i] = fRec155[i - 1] + fZec164[i];
			}
			/* Post code */
			for (int j163 = 0; j163 < 4; j163 = j163 + 1) {
				fRec154_perm[j163] = fRec154_tmp[vsize + j163];
			}
			for (int j165 = 0; j165 < 4; j165 = j165 + 1) {
				fRec155_perm[j165] = fRec155_tmp[vsize + j165];
			}
			/* Recursive loop 64 */
			/* Pre code */
			for (int j214 = 0; j214 < 4; j214 = j214 + 1) {
				fRec200_tmp[j214] = fRec200_perm[j214];
			}
			for (int j216 = 0; j216 < 4; j216 = j216 + 1) {
				fRec201_tmp[j216] = fRec201_perm[j216];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec220[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec219[i] + -1.0f) + fZec218[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec219[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec219[i] * (1.0f - fZec219[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec218[i])))) * ((fZec218[i] > 0.0f) ? 1.0f : ((fZec218[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec218[i]) - fSlow20 : tanhf(fZec218[i])))) - (fConst18 * fRec200[i - 1] + fRec201[i - 1]);
				fRec200[i] = fRec200[i - 1] + fConst21 * fZec220[i];
				fZec221[i] = fRec200[i - 1] + fConst20 * fZec220[i];
				fRec201[i] = fRec201[i - 1] + fConst22 * fZec221[i];
				fZec222[i] = fConst23 * fZec220[i];
				fRec202[i] = fZec222[i];
			}
			/* Post code */
			for (int j215 = 0; j215 < 4; j215 = j215 + 1) {
				fRec200_perm[j215] = fRec200_tmp[vsize + j215];
			}
			for (int j217 = 0; j217 < 4; j217 = j217 + 1) {
				fRec201_perm[j217] = fRec201_tmp[vsize + j217];
			}
			/* Vectorizable loop 65 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec29[i] = fSlow29 * fRec38[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 66 */
			/* Pre code */
			for (int j98 = 0; j98 < 4; j98 = j98 + 1) {
				fRec88_tmp[j98] = fRec88_perm[j98];
			}
			for (int j100 = 0; j100 < 4; j100 = j100 + 1) {
				fRec89_tmp[j100] = fRec89_perm[j100];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec95[i] = fRec93[i] - (fConst25 * fRec88[i - 1] + fRec89[i - 1]);
				fRec88[i] = fRec88[i - 1] + fConst28 * fZec95[i];
				fZec96[i] = fRec88[i - 1] + fConst27 * fZec95[i];
				fRec89[i] = fRec89[i - 1] + fConst29 * fZec96[i];
				fZec97[i] = fConst24 * fZec96[i];
				fRec90[i] = fRec89[i - 1] + fZec97[i];
			}
			/* Post code */
			for (int j99 = 0; j99 < 4; j99 = j99 + 1) {
				fRec88_perm[j99] = fRec88_tmp[vsize + j99];
			}
			for (int j101 = 0; j101 < 4; j101 = j101 + 1) {
				fRec89_perm[j101] = fRec89_tmp[vsize + j101];
			}
			/* Vectorizable loop 67 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec165[i] = fSlow29 * fRec156[i] * fZec10[i] + 0.03f;
			}
			/* Recursive loop 68 */
			/* Pre code */
			for (int j218 = 0; j218 < 4; j218 = j218 + 1) {
				fRec197_tmp[j218] = fRec197_perm[j218];
			}
			for (int j220 = 0; j220 < 4; j220 = j220 + 1) {
				fRec198_tmp[j220] = fRec198_perm[j220];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec223[i] = fRec202[i] - (fConst25 * fRec197[i - 1] + fRec198[i - 1]);
				fRec197[i] = fRec197[i - 1] + fConst28 * fZec223[i];
				fZec224[i] = fRec197[i - 1] + fConst27 * fZec223[i];
				fRec198[i] = fRec198[i - 1] + fConst29 * fZec224[i];
				fZec225[i] = fConst24 * fZec224[i];
				fRec199[i] = fRec198[i - 1] + fZec225[i];
			}
			/* Post code */
			for (int j219 = 0; j219 < 4; j219 = j219 + 1) {
				fRec197_perm[j219] = fRec197_tmp[vsize + j219];
			}
			for (int j221 = 0; j221 < 4; j221 = j221 + 1) {
				fRec198_perm[j221] = fRec198_tmp[vsize + j221];
			}
			/* Vectorizable loop 69 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec30[i] = mlczerov_faustpower2_f(fZec29[i]);
			}
			/* Recursive loop 70 */
			/* Pre code */
			for (int j50 = 0; j50 < 4; j50 = j50 + 1) {
				fRec61_tmp[j50] = fRec61_perm[j50];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec61[i] = fSlow33 + fConst2 * fRec61[i - 1];
			}
			/* Post code */
			for (int j51 = 0; j51 < 4; j51 = j51 + 1) {
				fRec61_perm[j51] = fRec61_tmp[vsize + j51];
			}
			/* Vectorizable loop 71 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec98[i] = fRec90[i] * fZec10[i];
			}
			/* Vectorizable loop 72 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec166[i] = mlczerov_faustpower2_f(fZec165[i]);
			}
			/* Vectorizable loop 73 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec226[i] = fRec199[i] * fZec10[i];
			}
			/* Recursive loop 74 */
			/* Pre code */
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec33_tmp[j38] = fRec33_perm[j38];
			}
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec34_tmp[j40] = fRec34_perm[j40];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec31[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec30[i] + -1.0f) + fZec29[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec30[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec30[i] * (1.0f - fZec30[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec29[i])))) * ((fZec29[i] > 0.0f) ? 1.0f : ((fZec29[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec29[i]) - fSlow20 : tanhf(fZec29[i])))) - (fConst18 * fRec33[i - 1] + fRec34[i - 1]);
				fRec33[i] = fRec33[i - 1] + fConst21 * fZec31[i];
				fZec32[i] = fRec33[i - 1] + fConst20 * fZec31[i];
				fRec34[i] = fRec34[i - 1] + fConst22 * fZec32[i];
				fZec33[i] = fConst23 * fZec31[i];
				fRec35[i] = fZec33[i];
			}
			/* Post code */
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec33_perm[j39] = fRec33_tmp[vsize + j39];
			}
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec34_perm[j41] = fRec34_tmp[vsize + j41];
			}
			/* Vectorizable loop 75 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec40[i] = 0.46f * fZec17[i];
			}
			/* Vectorizable loop 76 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec45[i] = std::pow(1e+01f, 0.05f * fRec61[i]);
			}
			/* Vectorizable loop 77 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec99[i] = 0.46f * fZec98[i];
			}
			/* Vectorizable loop 78 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec100[i] = mlczerov_faustpower2_f(fRec90[i]) * fZec15[i];
			}
			/* Recursive loop 79 */
			/* Pre code */
			for (int j166 = 0; j166 < 4; j166 = j166 + 1) {
				fRec151_tmp[j166] = fRec151_perm[j166];
			}
			for (int j168 = 0; j168 < 4; j168 = j168 + 1) {
				fRec152_tmp[j168] = fRec152_perm[j168];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec167[i] = 0.68f * ((iSlow27) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec166[i] + -1.0f) + fZec165[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec166[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec166[i] * (1.0f - fZec166[i]))) : ((iSlow28) ? (1.0f - std::exp(-(std::fabs(fZec165[i])))) * ((fZec165[i] > 0.0f) ? 1.0f : ((fZec165[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec165[i]) - fSlow20 : tanhf(fZec165[i])))) - (fConst18 * fRec151[i - 1] + fRec152[i - 1]);
				fRec151[i] = fRec151[i - 1] + fConst21 * fZec167[i];
				fZec168[i] = fRec151[i - 1] + fConst20 * fZec167[i];
				fRec152[i] = fRec152[i - 1] + fConst22 * fZec168[i];
				fZec169[i] = fConst23 * fZec167[i];
				fRec153[i] = fZec169[i];
			}
			/* Post code */
			for (int j167 = 0; j167 < 4; j167 = j167 + 1) {
				fRec151_perm[j167] = fRec151_tmp[vsize + j167];
			}
			for (int j169 = 0; j169 < 4; j169 = j169 + 1) {
				fRec152_perm[j169] = fRec152_tmp[vsize + j169];
			}
			/* Vectorizable loop 80 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec227[i] = 0.46f * fZec226[i];
			}
			/* Vectorizable loop 81 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec228[i] = mlczerov_faustpower2_f(fRec199[i]) * fZec15[i];
			}
			/* Recursive loop 82 */
			/* Pre code */
			for (int j42 = 0; j42 < 4; j42 = j42 + 1) {
				fRec30_tmp[j42] = fRec30_perm[j42];
			}
			for (int j44 = 0; j44 < 4; j44 = j44 + 1) {
				fRec31_tmp[j44] = fRec31_perm[j44];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec34[i] = fRec35[i] - (fConst25 * fRec30[i - 1] + fRec31[i - 1]);
				fRec30[i] = fRec30[i - 1] + fConst28 * fZec34[i];
				fZec35[i] = fRec30[i - 1] + fConst27 * fZec34[i];
				fRec31[i] = fRec31[i - 1] + fConst29 * fZec35[i];
				fZec36[i] = fConst24 * fZec35[i];
				fRec32[i] = fRec31[i - 1] + fZec36[i];
			}
			/* Post code */
			for (int j43 = 0; j43 < 4; j43 = j43 + 1) {
				fRec30_perm[j43] = fRec30_tmp[vsize + j43];
			}
			for (int j45 = 0; j45 < 4; j45 = j45 + 1) {
				fRec31_perm[j45] = fRec31_tmp[vsize + j45];
			}
			/* Vectorizable loop 83 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec46[i] = std::sqrt(fZec45[i]);
			}
			/* Recursive loop 84 */
			/* Pre code */
			for (int j56 = 0; j56 < 4; j56 = j56 + 1) {
				fRec62_tmp[j56] = fRec62_perm[j56];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec62[i] = fSlow34 + fConst2 * fRec62[i - 1];
			}
			/* Post code */
			for (int j57 = 0; j57 < 4; j57 = j57 + 1) {
				fRec62_perm[j57] = fRec62_tmp[vsize + j57];
			}
			/* Recursive loop 85 */
			/* Pre code */
			for (int j62 = 0; j62 < 4; j62 = j62 + 1) {
				fRec63_tmp[j62] = fRec63_perm[j62];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec63[i] = fSlow35 + fConst2 * fRec63[i - 1];
			}
			/* Post code */
			for (int j63 = 0; j63 < 4; j63 = j63 + 1) {
				fRec63_perm[j63] = fRec63_tmp[vsize + j63];
			}
			/* Recursive loop 86 */
			/* Pre code */
			for (int j102 = 0; j102 < 4; j102 = j102 + 1) {
				fRec83_tmp[j102] = fRec83_perm[j102];
			}
			for (int j104 = 0; j104 < 4; j104 = j104 + 1) {
				fRec84_tmp[j104] = fRec84_perm[j104];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec101[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec100[i] + -1.0f) + fZec98[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec100[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec100[i] * (0.35819647f * fZec100[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec99[i])))) * ((fZec99[i] > 0.0f) ? 1.0f : ((fZec99[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec99[i]) - fSlow20 : tanhf(fZec99[i])))) - (fConst31 * fRec83[i - 1] + fRec84[i - 1]);
				fRec83[i] = fRec83[i - 1] + fConst34 * fZec101[i];
				fZec102[i] = fRec83[i - 1] + fConst33 * fZec101[i];
				fRec84[i] = fRec84[i - 1] + fConst35 * fZec102[i];
				fZec103[i] = fConst30 * fZec102[i];
				fRec85[i] = fRec84[i - 1] + fZec103[i];
				fZec104[i] = fConst36 * fZec101[i];
				fRec86[i] = fZec104[i];
				fRec87[i] = fZec102[i];
			}
			/* Post code */
			for (int j103 = 0; j103 < 4; j103 = j103 + 1) {
				fRec83_perm[j103] = fRec83_tmp[vsize + j103];
			}
			for (int j105 = 0; j105 < 4; j105 = j105 + 1) {
				fRec84_perm[j105] = fRec84_tmp[vsize + j105];
			}
			/* Recursive loop 87 */
			/* Pre code */
			for (int j170 = 0; j170 < 4; j170 = j170 + 1) {
				fRec148_tmp[j170] = fRec148_perm[j170];
			}
			for (int j172 = 0; j172 < 4; j172 = j172 + 1) {
				fRec149_tmp[j172] = fRec149_perm[j172];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec170[i] = fRec153[i] - (fConst25 * fRec148[i - 1] + fRec149[i - 1]);
				fRec148[i] = fRec148[i - 1] + fConst28 * fZec170[i];
				fZec171[i] = fRec148[i - 1] + fConst27 * fZec170[i];
				fRec149[i] = fRec149[i - 1] + fConst29 * fZec171[i];
				fZec172[i] = fConst24 * fZec171[i];
				fRec150[i] = fRec149[i - 1] + fZec172[i];
			}
			/* Post code */
			for (int j171 = 0; j171 < 4; j171 = j171 + 1) {
				fRec148_perm[j171] = fRec148_tmp[vsize + j171];
			}
			for (int j173 = 0; j173 < 4; j173 = j173 + 1) {
				fRec149_perm[j173] = fRec149_tmp[vsize + j173];
			}
			/* Recursive loop 88 */
			/* Pre code */
			for (int j222 = 0; j222 < 4; j222 = j222 + 1) {
				fRec192_tmp[j222] = fRec192_perm[j222];
			}
			for (int j224 = 0; j224 < 4; j224 = j224 + 1) {
				fRec193_tmp[j224] = fRec193_perm[j224];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec229[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec228[i] + -1.0f) + fZec226[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec228[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec228[i] * (0.35819647f * fZec228[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec227[i])))) * ((fZec227[i] > 0.0f) ? 1.0f : ((fZec227[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec227[i]) - fSlow20 : tanhf(fZec227[i])))) - (fConst31 * fRec192[i - 1] + fRec193[i - 1]);
				fRec192[i] = fRec192[i - 1] + fConst34 * fZec229[i];
				fZec230[i] = fRec192[i - 1] + fConst33 * fZec229[i];
				fRec193[i] = fRec193[i - 1] + fConst35 * fZec230[i];
				fZec231[i] = fConst30 * fZec230[i];
				fRec194[i] = fRec193[i - 1] + fZec231[i];
				fZec232[i] = fConst36 * fZec229[i];
				fRec195[i] = fZec232[i];
				fRec196[i] = fZec230[i];
			}
			/* Post code */
			for (int j223 = 0; j223 < 4; j223 = j223 + 1) {
				fRec192_perm[j223] = fRec192_tmp[vsize + j223];
			}
			for (int j225 = 0; j225 < 4; j225 = j225 + 1) {
				fRec193_perm[j225] = fRec193_tmp[vsize + j225];
			}
			/* Vectorizable loop 89 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec37[i] = fRec32[i] * fZec10[i];
			}
			/* Vectorizable loop 90 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec51[i] = std::pow(1e+01f, 0.05f * (fRec62[i] + -2.5f));
			}
			/* Vectorizable loop 91 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec56[i] = std::pow(1e+01f, 0.05f * fRec63[i]);
			}
			/* Recursive loop 92 */
			/* Pre code */
			for (int j106 = 0; j106 < 4; j106 = j106 + 1) {
				fRec79_tmp[j106] = fRec79_perm[j106];
			}
			for (int j108 = 0; j108 < 4; j108 = j108 + 1) {
				fRec80_tmp[j108] = fRec80_perm[j108];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec105[i] = fRec86[i] + fRec85[i] * fZec45[i] + 1.4144272f * fRec87[i] * fZec46[i] - (fConst38 * fRec79[i - 1] + fRec80[i - 1]);
				fRec79[i] = fRec79[i - 1] + fConst41 * fZec105[i];
				fZec106[i] = fRec79[i - 1] + fConst40 * fZec105[i];
				fRec80[i] = fRec80[i - 1] + fConst42 * fZec106[i];
				fRec81[i] = fZec106[i];
				fZec107[i] = fConst43 * fZec105[i];
				fZec108[i] = fConst37 * fZec106[i];
				fRec82[i] = fZec108[i] + fRec80[i - 1] + fZec107[i];
			}
			/* Post code */
			for (int j107 = 0; j107 < 4; j107 = j107 + 1) {
				fRec79_perm[j107] = fRec79_tmp[vsize + j107];
			}
			for (int j109 = 0; j109 < 4; j109 = j109 + 1) {
				fRec80_perm[j109] = fRec80_tmp[vsize + j109];
			}
			/* Vectorizable loop 93 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec173[i] = fRec150[i] * fZec10[i];
			}
			/* Recursive loop 94 */
			/* Pre code */
			for (int j226 = 0; j226 < 4; j226 = j226 + 1) {
				fRec188_tmp[j226] = fRec188_perm[j226];
			}
			for (int j228 = 0; j228 < 4; j228 = j228 + 1) {
				fRec189_tmp[j228] = fRec189_perm[j228];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec233[i] = fRec195[i] + fRec194[i] * fZec45[i] + 1.4144272f * fRec196[i] * fZec46[i] - (fConst38 * fRec188[i - 1] + fRec189[i - 1]);
				fRec188[i] = fRec188[i - 1] + fConst41 * fZec233[i];
				fZec234[i] = fRec188[i - 1] + fConst40 * fZec233[i];
				fRec189[i] = fRec189[i - 1] + fConst42 * fZec234[i];
				fRec190[i] = fZec234[i];
				fZec235[i] = fConst43 * fZec233[i];
				fZec236[i] = fConst37 * fZec234[i];
				fRec191[i] = fZec236[i] + fRec189[i - 1] + fZec235[i];
			}
			/* Post code */
			for (int j227 = 0; j227 < 4; j227 = j227 + 1) {
				fRec188_perm[j227] = fRec188_tmp[vsize + j227];
			}
			for (int j229 = 0; j229 < 4; j229 = j229 + 1) {
				fRec189_perm[j229] = fRec189_tmp[vsize + j229];
			}
			/* Vectorizable loop 95 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec38[i] = 0.46f * fZec37[i];
			}
			/* Vectorizable loop 96 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec39[i] = mlczerov_faustpower2_f(fRec32[i]) * fZec15[i];
			}
			/* Vectorizable loop 97 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec57[i] = std::sqrt(fZec56[i]);
			}
			/* Recursive loop 98 */
			/* Pre code */
			for (int j110 = 0; j110 < 4; j110 = j110 + 1) {
				fRec74_tmp[j110] = fRec74_perm[j110];
			}
			for (int j112 = 0; j112 < 4; j112 = j112 + 1) {
				fRec75_tmp[j112] = fRec75_perm[j112];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec109[i] = fRec82[i] + fRec81[i] * fZec51[i] - (fConst45 * fRec74[i - 1] + fRec75[i - 1]);
				fRec74[i] = fRec74[i - 1] + fConst48 * fZec109[i];
				fZec110[i] = fRec74[i - 1] + fConst47 * fZec109[i];
				fRec75[i] = fRec75[i - 1] + fConst49 * fZec110[i];
				fZec111[i] = fConst44 * fZec110[i];
				fRec76[i] = fRec75[i - 1] + fZec111[i];
				fZec112[i] = fConst50 * fZec109[i];
				fRec77[i] = fZec112[i];
				fRec78[i] = fZec110[i];
			}
			/* Post code */
			for (int j111 = 0; j111 < 4; j111 = j111 + 1) {
				fRec74_perm[j111] = fRec74_tmp[vsize + j111];
			}
			for (int j113 = 0; j113 < 4; j113 = j113 + 1) {
				fRec75_perm[j113] = fRec75_tmp[vsize + j113];
			}
			/* Vectorizable loop 99 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec174[i] = 0.46f * fZec173[i];
			}
			/* Vectorizable loop 100 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec175[i] = mlczerov_faustpower2_f(fRec150[i]) * fZec15[i];
			}
			/* Recursive loop 101 */
			/* Pre code */
			for (int j230 = 0; j230 < 4; j230 = j230 + 1) {
				fRec183_tmp[j230] = fRec183_perm[j230];
			}
			for (int j232 = 0; j232 < 4; j232 = j232 + 1) {
				fRec184_tmp[j232] = fRec184_perm[j232];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec237[i] = fRec191[i] + fRec190[i] * fZec51[i] - (fConst45 * fRec183[i - 1] + fRec184[i - 1]);
				fRec183[i] = fRec183[i - 1] + fConst48 * fZec237[i];
				fZec238[i] = fRec183[i - 1] + fConst47 * fZec237[i];
				fRec184[i] = fRec184[i - 1] + fConst49 * fZec238[i];
				fZec239[i] = fConst44 * fZec238[i];
				fRec185[i] = fRec184[i - 1] + fZec239[i];
				fZec240[i] = fConst50 * fZec237[i];
				fRec186[i] = fZec240[i];
				fRec187[i] = fZec238[i];
			}
			/* Post code */
			for (int j231 = 0; j231 < 4; j231 = j231 + 1) {
				fRec183_perm[j231] = fRec183_tmp[vsize + j231];
			}
			for (int j233 = 0; j233 < 4; j233 = j233 + 1) {
				fRec184_perm[j233] = fRec184_tmp[vsize + j233];
			}
			/* Recursive loop 102 */
			/* Pre code */
			for (int j46 = 0; j46 < 4; j46 = j46 + 1) {
				fRec25_tmp[j46] = fRec25_perm[j46];
			}
			for (int j48 = 0; j48 < 4; j48 = j48 + 1) {
				fRec26_tmp[j48] = fRec26_perm[j48];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec41[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec39[i] + -1.0f) + fZec37[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec39[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec39[i] * (0.35819647f * fZec39[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec38[i])))) * ((fZec38[i] > 0.0f) ? 1.0f : ((fZec38[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec38[i]) - fSlow20 : tanhf(fZec38[i])))) - (fConst31 * fRec25[i - 1] + fRec26[i - 1]);
				fRec25[i] = fRec25[i - 1] + fConst34 * fZec41[i];
				fZec42[i] = fRec25[i - 1] + fConst33 * fZec41[i];
				fRec26[i] = fRec26[i - 1] + fConst35 * fZec42[i];
				fZec43[i] = fConst30 * fZec42[i];
				fRec27[i] = fRec26[i - 1] + fZec43[i];
				fZec44[i] = fConst36 * fZec41[i];
				fRec28[i] = fZec44[i];
				fRec29[i] = fZec42[i];
			}
			/* Post code */
			for (int j47 = 0; j47 < 4; j47 = j47 + 1) {
				fRec25_perm[j47] = fRec25_tmp[vsize + j47];
			}
			for (int j49 = 0; j49 < 4; j49 = j49 + 1) {
				fRec26_perm[j49] = fRec26_tmp[vsize + j49];
			}
			/* Vectorizable loop 103 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec113[i] = fRec76[i] + fRec77[i] * fZec56[i] + 1.4144272f * fRec78[i] * fZec57[i];
			}
			/* Recursive loop 104 */
			/* Pre code */
			for (int j174 = 0; j174 < 4; j174 = j174 + 1) {
				fRec143_tmp[j174] = fRec143_perm[j174];
			}
			for (int j176 = 0; j176 < 4; j176 = j176 + 1) {
				fRec144_tmp[j176] = fRec144_perm[j176];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec176[i] = 0.62f * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (0.4232f * fZec175[i] + -1.0f) + fZec173[i] * (fZec40[i] + 0.33f * fRec57[i] * (0.389344f * fZec175[i] + -1.38f)) + 0.25f * fRec56[i] * (fZec175[i] * (0.35819647f * fZec175[i] + -1.6928f) + 1.0f)) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec174[i])))) * ((fZec174[i] > 0.0f) ? 1.0f : ((fZec174[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec174[i]) - fSlow20 : tanhf(fZec174[i])))) - (fConst31 * fRec143[i - 1] + fRec144[i - 1]);
				fRec143[i] = fRec143[i - 1] + fConst34 * fZec176[i];
				fZec177[i] = fRec143[i - 1] + fConst33 * fZec176[i];
				fRec144[i] = fRec144[i - 1] + fConst35 * fZec177[i];
				fZec178[i] = fConst30 * fZec177[i];
				fRec145[i] = fRec144[i - 1] + fZec178[i];
				fZec179[i] = fConst36 * fZec176[i];
				fRec146[i] = fZec179[i];
				fRec147[i] = fZec177[i];
			}
			/* Post code */
			for (int j175 = 0; j175 < 4; j175 = j175 + 1) {
				fRec143_perm[j175] = fRec143_tmp[vsize + j175];
			}
			for (int j177 = 0; j177 < 4; j177 = j177 + 1) {
				fRec144_perm[j177] = fRec144_tmp[vsize + j177];
			}
			/* Vectorizable loop 105 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec241[i] = fRec185[i] + fRec186[i] * fZec56[i] + 1.4144272f * fRec187[i] * fZec57[i];
			}
			/* Recursive loop 106 */
			/* Pre code */
			for (int j52 = 0; j52 < 4; j52 = j52 + 1) {
				fRec21_tmp[j52] = fRec21_perm[j52];
			}
			for (int j54 = 0; j54 < 4; j54 = j54 + 1) {
				fRec22_tmp[j54] = fRec22_perm[j54];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec47[i] = fRec28[i] + fRec27[i] * fZec45[i] + 1.4144272f * fRec29[i] * fZec46[i] - (fConst38 * fRec21[i - 1] + fRec22[i - 1]);
				fRec21[i] = fRec21[i - 1] + fConst41 * fZec47[i];
				fZec48[i] = fRec21[i - 1] + fConst40 * fZec47[i];
				fRec22[i] = fRec22[i - 1] + fConst42 * fZec48[i];
				fRec23[i] = fZec48[i];
				fZec49[i] = fConst43 * fZec47[i];
				fZec50[i] = fConst37 * fZec48[i];
				fRec24[i] = fZec50[i] + fRec22[i - 1] + fZec49[i];
			}
			/* Post code */
			for (int j53 = 0; j53 < 4; j53 = j53 + 1) {
				fRec21_perm[j53] = fRec21_tmp[vsize + j53];
			}
			for (int j55 = 0; j55 < 4; j55 = j55 + 1) {
				fRec22_perm[j55] = fRec22_tmp[vsize + j55];
			}
			/* Recursive loop 107 */
			/* Pre code */
			for (int j64 = 0; j64 < 4; j64 = j64 + 1) {
				fRec64_tmp[j64] = fRec64_perm[j64];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec64[i] = fSlow36 + fConst2 * fRec64[i - 1];
			}
			/* Post code */
			for (int j65 = 0; j65 < 4; j65 = j65 + 1) {
				fRec64_perm[j65] = fRec64_tmp[vsize + j65];
			}
			/* Recursive loop 108 */
			/* Pre code */
			for (int j114 = 0; j114 < 4; j114 = j114 + 1) {
				fRec112_tmp[j114] = fRec112_perm[j114];
			}
			for (int j116 = 0; j116 < 4; j116 = j116 + 1) {
				fRec113_tmp[j116] = fRec113_perm[j116];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec114[i] = mlczerov_faustpower2_f(fZec113[i]) - (fConst52 * fRec112[i - 1] + fRec113[i - 1]);
				fRec112[i] = fRec112[i - 1] + fConst55 * fZec114[i];
				fZec115[i] = fRec112[i - 1] + fConst54 * fZec114[i];
				fRec113[i] = fRec113[i - 1] + fConst56 * fZec115[i];
				fZec116[i] = fConst51 * fZec115[i];
				fRec114[i] = fRec113[i - 1] + fZec116[i];
			}
			/* Post code */
			for (int j115 = 0; j115 < 4; j115 = j115 + 1) {
				fRec112_perm[j115] = fRec112_tmp[vsize + j115];
			}
			for (int j117 = 0; j117 < 4; j117 = j117 + 1) {
				fRec113_perm[j117] = fRec113_tmp[vsize + j117];
			}
			/* Recursive loop 109 */
			/* Pre code */
			for (int j178 = 0; j178 < 4; j178 = j178 + 1) {
				fRec139_tmp[j178] = fRec139_perm[j178];
			}
			for (int j180 = 0; j180 < 4; j180 = j180 + 1) {
				fRec140_tmp[j180] = fRec140_perm[j180];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec180[i] = fRec146[i] + fRec145[i] * fZec45[i] + 1.4144272f * fRec147[i] * fZec46[i] - (fConst38 * fRec139[i - 1] + fRec140[i - 1]);
				fRec139[i] = fRec139[i - 1] + fConst41 * fZec180[i];
				fZec181[i] = fRec139[i - 1] + fConst40 * fZec180[i];
				fRec140[i] = fRec140[i - 1] + fConst42 * fZec181[i];
				fRec141[i] = fZec181[i];
				fZec182[i] = fConst43 * fZec180[i];
				fZec183[i] = fConst37 * fZec181[i];
				fRec142[i] = fZec183[i] + fRec140[i - 1] + fZec182[i];
			}
			/* Post code */
			for (int j179 = 0; j179 < 4; j179 = j179 + 1) {
				fRec139_perm[j179] = fRec139_tmp[vsize + j179];
			}
			for (int j181 = 0; j181 < 4; j181 = j181 + 1) {
				fRec140_perm[j181] = fRec140_tmp[vsize + j181];
			}
			/* Recursive loop 110 */
			/* Pre code */
			for (int j234 = 0; j234 < 4; j234 = j234 + 1) {
				fRec221_tmp[j234] = fRec221_perm[j234];
			}
			for (int j236 = 0; j236 < 4; j236 = j236 + 1) {
				fRec222_tmp[j236] = fRec222_perm[j236];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec242[i] = mlczerov_faustpower2_f(fZec241[i]) - (fConst52 * fRec221[i - 1] + fRec222[i - 1]);
				fRec221[i] = fRec221[i - 1] + fConst55 * fZec242[i];
				fZec243[i] = fRec221[i - 1] + fConst54 * fZec242[i];
				fRec222[i] = fRec222[i - 1] + fConst56 * fZec243[i];
				fZec244[i] = fConst51 * fZec243[i];
				fRec223[i] = fRec222[i - 1] + fZec244[i];
			}
			/* Post code */
			for (int j235 = 0; j235 < 4; j235 = j235 + 1) {
				fRec221_perm[j235] = fRec221_tmp[vsize + j235];
			}
			for (int j237 = 0; j237 < 4; j237 = j237 + 1) {
				fRec222_perm[j237] = fRec222_tmp[vsize + j237];
			}
			/* Recursive loop 111 */
			/* Pre code */
			for (int j58 = 0; j58 < 4; j58 = j58 + 1) {
				fRec16_tmp[j58] = fRec16_perm[j58];
			}
			for (int j60 = 0; j60 < 4; j60 = j60 + 1) {
				fRec17_tmp[j60] = fRec17_perm[j60];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec52[i] = fRec24[i] + fRec23[i] * fZec51[i] - (fConst45 * fRec16[i - 1] + fRec17[i - 1]);
				fRec16[i] = fRec16[i - 1] + fConst48 * fZec52[i];
				fZec53[i] = fRec16[i - 1] + fConst47 * fZec52[i];
				fRec17[i] = fRec17[i - 1] + fConst49 * fZec53[i];
				fZec54[i] = fConst44 * fZec53[i];
				fRec18[i] = fRec17[i - 1] + fZec54[i];
				fZec55[i] = fConst50 * fZec52[i];
				fRec19[i] = fZec55[i];
				fRec20[i] = fZec53[i];
			}
			/* Post code */
			for (int j59 = 0; j59 < 4; j59 = j59 + 1) {
				fRec16_perm[j59] = fRec16_tmp[vsize + j59];
			}
			for (int j61 = 0; j61 < 4; j61 = j61 + 1) {
				fRec17_perm[j61] = fRec17_tmp[vsize + j61];
			}
			/* Recursive loop 112 */
			/* Pre code */
			for (int j118 = 0; j118 < 4; j118 = j118 + 1) {
				fRec70_tmp[j118] = fRec70_perm[j118];
			}
			for (int j120 = 0; j120 < 4; j120 = j120 + 1) {
				fRec71_tmp[j120] = fRec71_perm[j120];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec117[i] = fSlow38 * fZec113[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec114[i])) - (fConst58 * fRec70[i - 1] + fRec71[i - 1]);
				fRec70[i] = fRec70[i - 1] + fConst61 * fZec117[i];
				fZec118[i] = fRec70[i - 1] + fConst60 * fZec117[i];
				fRec71[i] = fRec71[i - 1] + fConst62 * fZec118[i];
				fRec72[i] = fZec118[i];
				fZec119[i] = fConst63 * fZec117[i];
				fZec120[i] = fConst57 * fZec118[i];
				fRec73[i] = fZec120[i] + fRec71[i - 1] + fZec119[i];
			}
			/* Post code */
			for (int j119 = 0; j119 < 4; j119 = j119 + 1) {
				fRec70_perm[j119] = fRec70_tmp[vsize + j119];
			}
			for (int j121 = 0; j121 < 4; j121 = j121 + 1) {
				fRec71_perm[j121] = fRec71_tmp[vsize + j121];
			}
			/* Recursive loop 113 */
			/* Pre code */
			for (int j182 = 0; j182 < 4; j182 = j182 + 1) {
				fRec134_tmp[j182] = fRec134_perm[j182];
			}
			for (int j184 = 0; j184 < 4; j184 = j184 + 1) {
				fRec135_tmp[j184] = fRec135_perm[j184];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec184[i] = fRec142[i] + fRec141[i] * fZec51[i] - (fConst45 * fRec134[i - 1] + fRec135[i - 1]);
				fRec134[i] = fRec134[i - 1] + fConst48 * fZec184[i];
				fZec185[i] = fRec134[i - 1] + fConst47 * fZec184[i];
				fRec135[i] = fRec135[i - 1] + fConst49 * fZec185[i];
				fZec186[i] = fConst44 * fZec185[i];
				fRec136[i] = fRec135[i - 1] + fZec186[i];
				fZec187[i] = fConst50 * fZec184[i];
				fRec137[i] = fZec187[i];
				fRec138[i] = fZec185[i];
			}
			/* Post code */
			for (int j183 = 0; j183 < 4; j183 = j183 + 1) {
				fRec134_perm[j183] = fRec134_tmp[vsize + j183];
			}
			for (int j185 = 0; j185 < 4; j185 = j185 + 1) {
				fRec135_perm[j185] = fRec135_tmp[vsize + j185];
			}
			/* Recursive loop 114 */
			/* Pre code */
			for (int j238 = 0; j238 < 4; j238 = j238 + 1) {
				fRec179_tmp[j238] = fRec179_perm[j238];
			}
			for (int j240 = 0; j240 < 4; j240 = j240 + 1) {
				fRec180_tmp[j240] = fRec180_perm[j240];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec245[i] = fSlow38 * fZec241[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec223[i])) - (fConst58 * fRec179[i - 1] + fRec180[i - 1]);
				fRec179[i] = fRec179[i - 1] + fConst61 * fZec245[i];
				fZec246[i] = fRec179[i - 1] + fConst60 * fZec245[i];
				fRec180[i] = fRec180[i - 1] + fConst62 * fZec246[i];
				fRec181[i] = fZec246[i];
				fZec247[i] = fConst63 * fZec245[i];
				fZec248[i] = fConst57 * fZec246[i];
				fRec182[i] = fZec248[i] + fRec180[i - 1] + fZec247[i];
			}
			/* Post code */
			for (int j239 = 0; j239 < 4; j239 = j239 + 1) {
				fRec179_perm[j239] = fRec179_tmp[vsize + j239];
			}
			for (int j241 = 0; j241 < 4; j241 = j241 + 1) {
				fRec180_perm[j241] = fRec180_tmp[vsize + j241];
			}
			/* Vectorizable loop 115 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec58[i] = fRec18[i] + fRec19[i] * fZec56[i] + 1.4144272f * fRec20[i] * fZec57[i];
			}
			/* Vectorizable loop 116 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec121[i] = fRec73[i] + fSlow39 * fRec72[i];
			}
			/* Vectorizable loop 117 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec188[i] = fRec136[i] + fRec137[i] * fZec56[i] + 1.4144272f * fRec138[i] * fZec57[i];
			}
			/* Vectorizable loop 118 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec249[i] = fRec182[i] + fSlow39 * fRec181[i];
			}
			/* Recursive loop 119 */
			/* Pre code */
			for (int j66 = 0; j66 < 4; j66 = j66 + 1) {
				fRec65_tmp[j66] = fRec65_perm[j66];
			}
			for (int j68 = 0; j68 < 4; j68 = j68 + 1) {
				fRec66_tmp[j68] = fRec66_perm[j68];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec59[i] = mlczerov_faustpower2_f(fZec58[i]) - (fConst52 * fRec65[i - 1] + fRec66[i - 1]);
				fRec65[i] = fRec65[i - 1] + fConst55 * fZec59[i];
				fZec60[i] = fRec65[i - 1] + fConst54 * fZec59[i];
				fRec66[i] = fRec66[i - 1] + fConst56 * fZec60[i];
				fZec61[i] = fConst51 * fZec60[i];
				fRec67[i] = fRec66[i - 1] + fZec61[i];
			}
			/* Post code */
			for (int j67 = 0; j67 < 4; j67 = j67 + 1) {
				fRec65_perm[j67] = fRec65_tmp[vsize + j67];
			}
			for (int j69 = 0; j69 < 4; j69 = j69 + 1) {
				fRec66_perm[j69] = fRec66_tmp[vsize + j69];
			}
			/* Vectorizable loop 120 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec122[i] = mlczerov_faustpower2_f(fZec121[i]);
			}
			/* Recursive loop 121 */
			/* Pre code */
			for (int j186 = 0; j186 < 4; j186 = j186 + 1) {
				fRec174_tmp[j186] = fRec174_perm[j186];
			}
			for (int j188 = 0; j188 < 4; j188 = j188 + 1) {
				fRec175_tmp[j188] = fRec175_perm[j188];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec189[i] = mlczerov_faustpower2_f(fZec188[i]) - (fConst52 * fRec174[i - 1] + fRec175[i - 1]);
				fRec174[i] = fRec174[i - 1] + fConst55 * fZec189[i];
				fZec190[i] = fRec174[i - 1] + fConst54 * fZec189[i];
				fRec175[i] = fRec175[i - 1] + fConst56 * fZec190[i];
				fZec191[i] = fConst51 * fZec190[i];
				fRec176[i] = fRec175[i - 1] + fZec191[i];
			}
			/* Post code */
			for (int j187 = 0; j187 < 4; j187 = j187 + 1) {
				fRec174_perm[j187] = fRec174_tmp[vsize + j187];
			}
			for (int j189 = 0; j189 < 4; j189 = j189 + 1) {
				fRec175_perm[j189] = fRec175_tmp[vsize + j189];
			}
			/* Vectorizable loop 122 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec250[i] = mlczerov_faustpower2_f(fZec249[i]);
			}
			/* Recursive loop 123 */
			/* Pre code */
			for (int j70 = 0; j70 < 4; j70 = j70 + 1) {
				fRec12_tmp[j70] = fRec12_perm[j70];
			}
			for (int j72 = 0; j72 < 4; j72 = j72 + 1) {
				fRec13_tmp[j72] = fRec13_perm[j72];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec62[i] = fSlow38 * fZec58[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec67[i])) - (fConst58 * fRec12[i - 1] + fRec13[i - 1]);
				fRec12[i] = fRec12[i - 1] + fConst61 * fZec62[i];
				fZec63[i] = fRec12[i - 1] + fConst60 * fZec62[i];
				fRec13[i] = fRec13[i - 1] + fConst62 * fZec63[i];
				fRec14[i] = fZec63[i];
				fZec64[i] = fConst63 * fZec62[i];
				fZec65[i] = fConst57 * fZec63[i];
				fRec15[i] = fZec65[i] + fRec13[i - 1] + fZec64[i];
			}
			/* Post code */
			for (int j71 = 0; j71 < 4; j71 = j71 + 1) {
				fRec12_perm[j71] = fRec12_tmp[vsize + j71];
			}
			for (int j73 = 0; j73 < 4; j73 = j73 + 1) {
				fRec13_perm[j73] = fRec13_tmp[vsize + j73];
			}
			/* Vectorizable loop 124 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec123[i] = ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec122[i] + -1.0f) + fZec121[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec122[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec122[i] * (1.0f - fZec122[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec121[i])))) * ((fZec121[i] > 0.0f) ? 1.0f : ((fZec121[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec121[i]) - fSlow20 : tanhf(fZec121[i]))));
			}
			/* Recursive loop 125 */
			/* Pre code */
			for (int j190 = 0; j190 < 4; j190 = j190 + 1) {
				fRec130_tmp[j190] = fRec130_perm[j190];
			}
			for (int j192 = 0; j192 < 4; j192 = j192 + 1) {
				fRec131_tmp[j192] = fRec131_perm[j192];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec192[i] = fSlow38 * fZec188[i] * (1.0f - 0.7f * fRec64[i] * std::min<float>(1.0f, fRec176[i])) - (fConst58 * fRec130[i - 1] + fRec131[i - 1]);
				fRec130[i] = fRec130[i - 1] + fConst61 * fZec192[i];
				fZec193[i] = fRec130[i - 1] + fConst60 * fZec192[i];
				fRec131[i] = fRec131[i - 1] + fConst62 * fZec193[i];
				fRec132[i] = fZec193[i];
				fZec194[i] = fConst63 * fZec192[i];
				fZec195[i] = fConst57 * fZec193[i];
				fRec133[i] = fZec195[i] + fRec131[i - 1] + fZec194[i];
			}
			/* Post code */
			for (int j191 = 0; j191 < 4; j191 = j191 + 1) {
				fRec130_perm[j191] = fRec130_tmp[vsize + j191];
			}
			for (int j193 = 0; j193 < 4; j193 = j193 + 1) {
				fRec131_perm[j193] = fRec131_tmp[vsize + j193];
			}
			/* Vectorizable loop 126 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec251[i] = ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec250[i] + -1.0f) + fZec249[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec250[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec250[i] * (1.0f - fZec250[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec249[i])))) * ((fZec249[i] > 0.0f) ? 1.0f : ((fZec249[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec249[i]) - fSlow20 : tanhf(fZec249[i]))));
			}
			/* Recursive loop 127 */
			/* Pre code */
			for (int j122 = 0; j122 < 4; j122 = j122 + 1) {
				fRec69_tmp[j122] = fRec69_perm[j122];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec69[i] = std::max<float>(0.995f * fRec69[i - 1], std::fabs(fSlow40 * fZec123[i]));
			}
			/* Post code */
			for (int j123 = 0; j123 < 4; j123 = j123 + 1) {
				fRec69_perm[j123] = fRec69_tmp[vsize + j123];
			}
			/* Vectorizable loop 128 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec124[i] = fRec15[i] + fSlow39 * fRec14[i];
			}
			/* Recursive loop 129 */
			/* Pre code */
			for (int j130 = 0; j130 < 4; j130 = j130 + 1) {
				fRec115_tmp[j130] = fRec115_perm[j130];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec115[i] = fSlow42 + fConst2 * fRec115[i - 1];
			}
			/* Post code */
			for (int j131 = 0; j131 < 4; j131 = j131 + 1) {
				fRec115_perm[j131] = fRec115_tmp[vsize + j131];
			}
			/* Recursive loop 130 */
			/* Pre code */
			for (int j242 = 0; j242 < 4; j242 = j242 + 1) {
				fRec178_tmp[j242] = fRec178_perm[j242];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec178[i] = std::max<float>(0.995f * fRec178[i - 1], std::fabs(fSlow40 * fZec251[i]));
			}
			/* Post code */
			for (int j243 = 0; j243 < 4; j243 = j243 + 1) {
				fRec178_perm[j243] = fRec178_tmp[vsize + j243];
			}
			/* Vectorizable loop 131 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec252[i] = fRec133[i] + fSlow39 * fRec132[i];
			}
			/* Recursive loop 132 */
			/* Pre code */
			for (int j124 = 0; j124 < 4; j124 = j124 + 1) {
				fRec68_tmp[j124] = fRec68_perm[j124];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec68[i] = fConst1 * static_cast<float>(fRec69[i] > fZec0[i]) + fConst2 * fRec68[i - 1];
			}
			/* Post code */
			for (int j125 = 0; j125 < 4; j125 = j125 + 1) {
				fRec68_perm[j125] = fRec68_tmp[vsize + j125];
			}
			/* Vectorizable loop 133 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec125[i] = mlczerov_faustpower2_f(fZec124[i]);
			}
			/* Vectorizable loop 134 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec130[i] = std::pow(1e+01f, fSlow44 * fRec115[i]);
			}
			/* Recursive loop 135 */
			/* Pre code */
			for (int j136 = 0; j136 < 4; j136 = j136 + 1) {
				fRec116_tmp[j136] = fRec116_perm[j136];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec116[i] = fSlow45 + fConst2 * fRec116[i - 1];
			}
			/* Post code */
			for (int j137 = 0; j137 < 4; j137 = j137 + 1) {
				fRec116_perm[j137] = fRec116_tmp[vsize + j137];
			}
			/* Recursive loop 136 */
			/* Pre code */
			for (int j244 = 0; j244 < 4; j244 = j244 + 1) {
				fRec177_tmp[j244] = fRec177_perm[j244];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec177[i] = fConst1 * static_cast<float>(fRec178[i] > fZec0[i]) + fConst2 * fRec177[i - 1];
			}
			/* Post code */
			for (int j245 = 0; j245 < 4; j245 = j245 + 1) {
				fRec177_perm[j245] = fRec177_tmp[vsize + j245];
			}
			/* Vectorizable loop 137 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec253[i] = mlczerov_faustpower2_f(fZec252[i]);
			}
			/* Recursive loop 138 */
			/* Pre code */
			for (int j126 = 0; j126 < 4; j126 = j126 + 1) {
				fRec7_tmp[j126] = fRec7_perm[j126];
			}
			for (int j128 = 0; j128 < 4; j128 = j128 + 1) {
				fRec8_tmp[j128] = fRec8_perm[j128];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec126[i] = ((iSlow41) ? fSlow40 * fRec68[i] * fZec123[i] : fSlow40 * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec125[i] + -1.0f) + fZec124[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec125[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec125[i] * (1.0f - fZec125[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec124[i])))) * ((fZec124[i] > 0.0f) ? 1.0f : ((fZec124[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec124[i]) - fSlow20 : tanhf(fZec124[i]))))) - (fConst65 * fRec7[i - 1] + fRec8[i - 1]);
				fRec7[i] = fRec7[i - 1] + fConst68 * fZec126[i];
				fZec127[i] = fRec7[i - 1] + fConst67 * fZec126[i];
				fRec8[i] = fRec8[i - 1] + fConst69 * fZec127[i];
				fZec128[i] = fConst64 * fZec127[i];
				fRec9[i] = fRec8[i - 1] + fZec128[i];
				fZec129[i] = fConst70 * fZec126[i];
				fRec10[i] = fZec129[i];
				fRec11[i] = fZec127[i];
			}
			/* Post code */
			for (int j127 = 0; j127 < 4; j127 = j127 + 1) {
				fRec7_perm[j127] = fRec7_tmp[vsize + j127];
			}
			for (int j129 = 0; j129 < 4; j129 = j129 + 1) {
				fRec8_perm[j129] = fRec8_tmp[vsize + j129];
			}
			/* Recursive loop 139 */
			/* Pre code */
			for (int j2 = 0; j2 < 4; j2 = j2 + 1) {
				fRec1_tmp[j2] = fRec1_perm[j2];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec1[i] = fSlow1 + fConst2 * fRec1[i - 1];
			}
			/* Post code */
			for (int j3 = 0; j3 < 4; j3 = j3 + 1) {
				fRec1_perm[j3] = fRec1_tmp[vsize + j3];
			}
			/* Vectorizable loop 140 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec131[i] = std::sqrt(fZec130[i]);
			}
			/* Vectorizable loop 141 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec139[i] = std::pow(1e+01f, fSlow46 * fRec116[i]);
			}
			/* Recursive loop 142 */
			/* Pre code */
			for (int j246 = 0; j246 < 4; j246 = j246 + 1) {
				fRec125_tmp[j246] = fRec125_perm[j246];
			}
			for (int j248 = 0; j248 < 4; j248 = j248 + 1) {
				fRec126_tmp[j248] = fRec126_perm[j248];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec254[i] = ((iSlow41) ? fSlow40 * fRec177[i] * fZec251[i] : fSlow40 * ((iSlow31) ? tanhf(0.5f * fRec55[i] * (2.0f * fZec253[i] + -1.0f) + fZec252[i] * (fZec17[i] + 0.33f * fRec57[i] * (4.0f * fZec253[i] + -3.0f)) + 0.25f * fRec56[i] * (1.0f - 8.0f * fZec253[i] * (1.0f - fZec253[i]))) : ((iSlow32) ? (1.0f - std::exp(-(std::fabs(fZec252[i])))) * ((fZec252[i] > 0.0f) ? 1.0f : ((fZec252[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow13) ? tanhf(fSlow19 + fZec252[i]) - fSlow20 : tanhf(fZec252[i]))))) - (fConst65 * fRec125[i - 1] + fRec126[i - 1]);
				fRec125[i] = fRec125[i - 1] + fConst68 * fZec254[i];
				fZec255[i] = fRec125[i - 1] + fConst67 * fZec254[i];
				fRec126[i] = fRec126[i - 1] + fConst69 * fZec255[i];
				fZec256[i] = fConst64 * fZec255[i];
				fRec127[i] = fRec126[i - 1] + fZec256[i];
				fZec257[i] = fConst70 * fZec254[i];
				fRec128[i] = fZec257[i];
				fRec129[i] = fZec255[i];
			}
			/* Post code */
			for (int j247 = 0; j247 < 4; j247 = j247 + 1) {
				fRec125_perm[j247] = fRec125_tmp[vsize + j247];
			}
			for (int j249 = 0; j249 < 4; j249 = j249 + 1) {
				fRec126_perm[j249] = fRec126_tmp[vsize + j249];
			}
			/* Recursive loop 143 */
			/* Pre code */
			for (int j132 = 0; j132 < 4; j132 = j132 + 1) {
				fRec2_tmp[j132] = fRec2_perm[j132];
			}
			for (int j134 = 0; j134 < 4; j134 = j134 + 1) {
				fRec3_tmp[j134] = fRec3_perm[j134];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec132[i] = fRec10[i] + fRec9[i] * fZec130[i] + 1.25f * fRec11[i] * fZec131[i] - (fConst72 * fRec2[i - 1] + fRec3[i - 1]);
				fRec2[i] = fRec2[i - 1] + fConst75 * fZec132[i];
				fZec133[i] = fRec2[i - 1] + fConst74 * fZec132[i];
				fRec3[i] = fRec3[i - 1] + fConst76 * fZec133[i];
				fZec134[i] = fConst71 * fZec133[i];
				fRec4[i] = fRec3[i - 1] + fZec134[i];
				fZec135[i] = fConst77 * fZec132[i];
				fRec5[i] = fZec135[i];
				fRec6[i] = fZec133[i];
			}
			/* Post code */
			for (int j133 = 0; j133 < 4; j133 = j133 + 1) {
				fRec2_perm[j133] = fRec2_tmp[vsize + j133];
			}
			for (int j135 = 0; j135 < 4; j135 = j135 + 1) {
				fRec3_perm[j135] = fRec3_tmp[vsize + j135];
			}
			/* Recursive loop 144 */
			/* Pre code */
			for (int j0 = 0; j0 < 4; j0 = j0 + 1) {
				fRec0_tmp[j0] = fRec0_perm[j0];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec0[i] = fSlow0 + fConst2 * fRec0[i - 1];
			}
			/* Post code */
			for (int j1 = 0; j1 < 4; j1 = j1 + 1) {
				fRec0_perm[j1] = fRec0_tmp[vsize + j1];
			}
			/* Recursive loop 145 */
			/* Pre code */
			for (int j138 = 0; j138 < 4; j138 = j138 + 1) {
				fRec117_tmp[j138] = fRec117_perm[j138];
			}
			for (int j140 = 0; j140 < 4; j140 = j140 + 1) {
				fRec118_tmp[j140] = fRec118_perm[j140];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec136[i] = static_cast<float>(input0[i]) - (fConst79 * fRec117[i - 1] + fRec118[i - 1]);
				fRec117[i] = fRec117[i - 1] + fConst82 * fZec136[i];
				fZec137[i] = fRec117[i - 1] + fConst81 * fZec136[i];
				fRec118[i] = fRec118[i - 1] + fConst83 * fZec137[i];
				fZec138[i] = fConst78 * fZec137[i];
				fRec119[i] = fRec118[i - 1] + fZec138[i];
			}
			/* Post code */
			for (int j139 = 0; j139 < 4; j139 = j139 + 1) {
				fRec117_perm[j139] = fRec117_tmp[vsize + j139];
			}
			for (int j141 = 0; j141 < 4; j141 = j141 + 1) {
				fRec118_perm[j141] = fRec118_tmp[vsize + j141];
			}
			/* Vectorizable loop 146 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec140[i] = std::sqrt(fZec139[i]);
			}
			/* Vectorizable loop 147 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec141[i] = 1.0f - fRec1[i];
			}
			/* Recursive loop 148 */
			/* Pre code */
			for (int j250 = 0; j250 < 4; j250 = j250 + 1) {
				fRec120_tmp[j250] = fRec120_perm[j250];
			}
			for (int j252 = 0; j252 < 4; j252 = j252 + 1) {
				fRec121_tmp[j252] = fRec121_perm[j252];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec258[i] = fRec128[i] + fRec127[i] * fZec130[i] + 1.25f * fRec129[i] * fZec131[i] - (fConst72 * fRec120[i - 1] + fRec121[i - 1]);
				fRec120[i] = fRec120[i - 1] + fConst75 * fZec258[i];
				fZec259[i] = fRec120[i - 1] + fConst74 * fZec258[i];
				fRec121[i] = fRec121[i - 1] + fConst76 * fZec259[i];
				fZec260[i] = fConst71 * fZec259[i];
				fRec122[i] = fRec121[i - 1] + fZec260[i];
				fZec261[i] = fConst77 * fZec258[i];
				fRec123[i] = fZec261[i];
				fRec124[i] = fZec259[i];
			}
			/* Post code */
			for (int j251 = 0; j251 < 4; j251 = j251 + 1) {
				fRec120_perm[j251] = fRec120_tmp[vsize + j251];
			}
			for (int j253 = 0; j253 < 4; j253 = j253 + 1) {
				fRec121_perm[j253] = fRec121_tmp[vsize + j253];
			}
			/* Recursive loop 149 */
			/* Pre code */
			for (int j254 = 0; j254 < 4; j254 = j254 + 1) {
				fRec224_tmp[j254] = fRec224_perm[j254];
			}
			for (int j256 = 0; j256 < 4; j256 = j256 + 1) {
				fRec225_tmp[j256] = fRec225_perm[j256];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec262[i] = static_cast<float>(input1[i]) - (fConst79 * fRec224[i - 1] + fRec225[i - 1]);
				fRec224[i] = fRec224[i - 1] + fConst82 * fZec262[i];
				fZec263[i] = fRec224[i - 1] + fConst81 * fZec262[i];
				fRec225[i] = fRec225[i - 1] + fConst83 * fZec263[i];
				fZec264[i] = fConst78 * fZec263[i];
				fRec226[i] = fRec225[i - 1] + fZec264[i];
			}
			/* Post code */
			for (int j255 = 0; j255 < 4; j255 = j255 + 1) {
				fRec224_perm[j255] = fRec224_tmp[vsize + j255];
			}
			for (int j257 = 0; j257 < 4; j257 = j257 + 1) {
				fRec225_perm[j257] = fRec225_tmp[vsize + j257];
			}
			/* Vectorizable loop 150 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fZec141[i] * (fRec4[i] + fRec5[i] * fZec139[i] + 1.4285715f * fRec6[i] * fZec140[i]) + fRec1[i] * fRec119[i]));
			}
			/* Vectorizable loop 151 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fZec141[i] * (fRec122[i] + fRec123[i] * fZec139[i] + 1.4285715f * fRec124[i] * fZec140[i]) + fRec1[i] * fRec226[i]));
			}
		}
	}

};

#endif
