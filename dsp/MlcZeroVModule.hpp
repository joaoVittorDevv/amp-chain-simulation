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


class mlczerov : public dsp {
	
 private:
	
	int fSampleRate;
	float fConst0;
	float fConst1;
	float fConst2;
	FAUSTFLOAT fHslider0;
	float fRec0_perm[4];
	FAUSTFLOAT fHslider1;
	float fRec29_perm[4];
	float fRec40_perm[4];
	FAUSTFLOAT fHslider2;
	float fRec41_perm[4];
	float fRec39_perm[4];
	float fConst3;
	float fConst4;
	float fConst5;
	float fConst6;
	float fConst7;
	float fRec34_perm[4];
	float fConst8;
	float fRec35_perm[4];
	float fConst9;
	float fConst10;
	float fConst11;
	FAUSTFLOAT fEntry0;
	FAUSTFLOAT fEntry1;
	float fConst12;
	float fConst13;
	float fConst14;
	float fRec30_perm[4];
	float fConst15;
	float fRec31_perm[4];
	float fConst16;
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
	float fRec42_perm[4];
	float fConst22;
	float fRec43_perm[4];
	float fConst23;
	float fConst24;
	float fConst25;
	FAUSTFLOAT fEntry8;
	float fConst26;
	float fConst27;
	float fConst28;
	float fRec24_perm[4];
	float fConst29;
	float fRec25_perm[4];
	float fConst30;
	FAUSTFLOAT fHslider3;
	float fRec45_perm[4];
	float fConst31;
	float fConst32;
	float fConst33;
	float fConst34;
	float fConst35;
	float fRec20_perm[4];
	float fConst36;
	float fRec21_perm[4];
	float fConst37;
	FAUSTFLOAT fHslider4;
	float fRec46_perm[4];
	float fConst38;
	float fConst39;
	float fConst40;
	float fConst41;
	float fConst42;
	float fRec15_perm[4];
	float fConst43;
	float fRec16_perm[4];
	float fConst44;
	FAUSTFLOAT fHslider5;
	float fRec47_perm[4];
	float fConst45;
	float fConst46;
	FAUSTFLOAT fEntry9;
	float fConst47;
	float fConst48;
	float fConst49;
	float fRec11_perm[4];
	float fConst50;
	float fRec12_perm[4];
	float fConst51;
	float fRec72_perm[4];
	float fRec73_perm[4];
	float fRec68_perm[4];
	float fRec69_perm[4];
	float fRec77_perm[4];
	float fRec78_perm[4];
	float fRec63_perm[4];
	float fRec64_perm[4];
	float fRec59_perm[4];
	float fRec60_perm[4];
	float fRec54_perm[4];
	float fRec55_perm[4];
	float fRec50_perm[4];
	float fRec51_perm[4];
	float fRec49_perm[4];
	float fRec48_perm[4];
	float fConst52;
	float fConst53;
	FAUSTFLOAT fEntry10;
	float fConst54;
	float fConst55;
	float fConst56;
	float fRec6_perm[4];
	float fConst57;
	float fRec7_perm[4];
	float fConst58;
	FAUSTFLOAT fHslider6;
	float fRec80_perm[4];
	float fConst59;
	float fConst60;
	FAUSTFLOAT fEntry11;
	float fConst61;
	float fConst62;
	float fConst63;
	float fRec1_perm[4];
	float fConst64;
	float fRec2_perm[4];
	float fConst65;
	FAUSTFLOAT fHslider7;
	float fRec81_perm[4];
	float fRec120_perm[4];
	float fRec119_perm[4];
	float fRec114_perm[4];
	float fRec115_perm[4];
	float fRec110_perm[4];
	float fRec111_perm[4];
	float fRec121_perm[4];
	float fRec122_perm[4];
	float fRec105_perm[4];
	float fRec106_perm[4];
	float fRec101_perm[4];
	float fRec102_perm[4];
	float fRec96_perm[4];
	float fRec97_perm[4];
	float fRec92_perm[4];
	float fRec93_perm[4];
	float fRec148_perm[4];
	float fRec149_perm[4];
	float fRec144_perm[4];
	float fRec145_perm[4];
	float fRec153_perm[4];
	float fRec154_perm[4];
	float fRec139_perm[4];
	float fRec140_perm[4];
	float fRec135_perm[4];
	float fRec136_perm[4];
	float fRec130_perm[4];
	float fRec131_perm[4];
	float fRec126_perm[4];
	float fRec127_perm[4];
	float fRec125_perm[4];
	float fRec124_perm[4];
	float fRec87_perm[4];
	float fRec88_perm[4];
	float fRec82_perm[4];
	float fRec83_perm[4];
	
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
		fConst24 = std::tan(376.99112f / fConst0);
		fConst25 = fConst24 + 1.4144272f;
		fConst26 = fConst24 * fConst25 + 1.0f;
		fConst27 = fConst24 / fConst26;
		fConst28 = 2.0f * fConst27;
		fConst29 = 2.0f * fConst24;
		fConst30 = 1.0f / fConst26;
		fConst31 = std::tan(2387.6104f / fConst0);
		fConst32 = fConst31 + 1.1764706f;
		fConst33 = fConst31 * fConst32 + 1.0f;
		fConst34 = fConst31 / fConst33;
		fConst35 = 2.0f * fConst34;
		fConst36 = 2.0f * fConst31;
		fConst37 = 1.0f / fConst33;
		fConst38 = std::tan(10681.415f / fConst0);
		fConst39 = fConst38 + 1.4144272f;
		fConst40 = fConst38 * fConst39 + 1.0f;
		fConst41 = fConst38 / fConst40;
		fConst42 = 2.0f * fConst41;
		fConst43 = 2.0f * fConst38;
		fConst44 = 1.0f / fConst40;
		fConst45 = std::tan(2984.513f / fConst0);
		fConst46 = fConst45 + 0.8333333f;
		fConst47 = fConst45 * fConst46 + 1.0f;
		fConst48 = fConst45 / fConst47;
		fConst49 = 2.0f * fConst48;
		fConst50 = 2.0f * fConst45;
		fConst51 = 1.0f / fConst47;
		fConst52 = std::tan(298.4513f / fConst0);
		fConst53 = fConst52 + 1.25f;
		fConst54 = fConst52 * fConst53 + 1.0f;
		fConst55 = fConst52 / fConst54;
		fConst56 = 2.0f * fConst55;
		fConst57 = 2.0f * fConst52;
		fConst58 = 1.0f / fConst54;
		fConst59 = std::tan(11309.733f / fConst0);
		fConst60 = fConst59 + 1.4285715f;
		fConst61 = fConst59 * fConst60 + 1.0f;
		fConst62 = fConst59 / fConst61;
		fConst63 = 2.0f * fConst62;
		fConst64 = 2.0f * fConst59;
		fConst65 = 1.0f / fConst61;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(0.5011872f);
		fHslider1 = static_cast<FAUSTFLOAT>(0.25118864f);
		fHslider2 = static_cast<FAUSTFLOAT>(-8e+01f);
		fEntry0 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry1 = static_cast<FAUSTFLOAT>(-3.0f);
		fEntry2 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry3 = static_cast<FAUSTFLOAT>(1.0f);
		fEntry4 = static_cast<FAUSTFLOAT>(3.0f);
		fEntry5 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry6 = static_cast<FAUSTFLOAT>(1.0f);
		fEntry7 = static_cast<FAUSTFLOAT>(0.5f);
		fEntry8 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider5 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry9 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry10 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider6 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry11 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider7 = static_cast<FAUSTFLOAT>(0.0f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 4; l0 = l0 + 1) {
			fRec0_perm[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 4; l1 = l1 + 1) {
			fRec29_perm[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 4; l2 = l2 + 1) {
			fRec40_perm[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 4; l3 = l3 + 1) {
			fRec41_perm[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 4; l4 = l4 + 1) {
			fRec39_perm[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 4; l5 = l5 + 1) {
			fRec34_perm[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 4; l6 = l6 + 1) {
			fRec35_perm[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 4; l7 = l7 + 1) {
			fRec30_perm[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 4; l8 = l8 + 1) {
			fRec31_perm[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 4; l9 = l9 + 1) {
			fRec42_perm[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 4; l10 = l10 + 1) {
			fRec43_perm[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 4; l11 = l11 + 1) {
			fRec24_perm[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 4; l12 = l12 + 1) {
			fRec25_perm[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 4; l13 = l13 + 1) {
			fRec45_perm[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 4; l14 = l14 + 1) {
			fRec20_perm[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 4; l15 = l15 + 1) {
			fRec21_perm[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 4; l16 = l16 + 1) {
			fRec46_perm[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 4; l17 = l17 + 1) {
			fRec15_perm[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 4; l18 = l18 + 1) {
			fRec16_perm[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 4; l19 = l19 + 1) {
			fRec47_perm[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 4; l20 = l20 + 1) {
			fRec11_perm[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 4; l21 = l21 + 1) {
			fRec12_perm[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 4; l22 = l22 + 1) {
			fRec72_perm[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 4; l23 = l23 + 1) {
			fRec73_perm[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 4; l24 = l24 + 1) {
			fRec68_perm[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 4; l25 = l25 + 1) {
			fRec69_perm[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 4; l26 = l26 + 1) {
			fRec77_perm[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 4; l27 = l27 + 1) {
			fRec78_perm[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 4; l28 = l28 + 1) {
			fRec63_perm[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 4; l29 = l29 + 1) {
			fRec64_perm[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 4; l30 = l30 + 1) {
			fRec59_perm[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 4; l31 = l31 + 1) {
			fRec60_perm[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 4; l32 = l32 + 1) {
			fRec54_perm[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 4; l33 = l33 + 1) {
			fRec55_perm[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 4; l34 = l34 + 1) {
			fRec50_perm[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 4; l35 = l35 + 1) {
			fRec51_perm[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 4; l36 = l36 + 1) {
			fRec49_perm[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 4; l37 = l37 + 1) {
			fRec48_perm[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 4; l38 = l38 + 1) {
			fRec6_perm[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 4; l39 = l39 + 1) {
			fRec7_perm[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 4; l40 = l40 + 1) {
			fRec80_perm[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 4; l41 = l41 + 1) {
			fRec1_perm[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 4; l42 = l42 + 1) {
			fRec2_perm[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 4; l43 = l43 + 1) {
			fRec81_perm[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 4; l44 = l44 + 1) {
			fRec120_perm[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 4; l45 = l45 + 1) {
			fRec119_perm[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 4; l46 = l46 + 1) {
			fRec114_perm[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 4; l47 = l47 + 1) {
			fRec115_perm[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 4; l48 = l48 + 1) {
			fRec110_perm[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 4; l49 = l49 + 1) {
			fRec111_perm[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 4; l50 = l50 + 1) {
			fRec121_perm[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 4; l51 = l51 + 1) {
			fRec122_perm[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 4; l52 = l52 + 1) {
			fRec105_perm[l52] = 0.0f;
		}
		for (int l53 = 0; l53 < 4; l53 = l53 + 1) {
			fRec106_perm[l53] = 0.0f;
		}
		for (int l54 = 0; l54 < 4; l54 = l54 + 1) {
			fRec101_perm[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 4; l55 = l55 + 1) {
			fRec102_perm[l55] = 0.0f;
		}
		for (int l56 = 0; l56 < 4; l56 = l56 + 1) {
			fRec96_perm[l56] = 0.0f;
		}
		for (int l57 = 0; l57 < 4; l57 = l57 + 1) {
			fRec97_perm[l57] = 0.0f;
		}
		for (int l58 = 0; l58 < 4; l58 = l58 + 1) {
			fRec92_perm[l58] = 0.0f;
		}
		for (int l59 = 0; l59 < 4; l59 = l59 + 1) {
			fRec93_perm[l59] = 0.0f;
		}
		for (int l60 = 0; l60 < 4; l60 = l60 + 1) {
			fRec148_perm[l60] = 0.0f;
		}
		for (int l61 = 0; l61 < 4; l61 = l61 + 1) {
			fRec149_perm[l61] = 0.0f;
		}
		for (int l62 = 0; l62 < 4; l62 = l62 + 1) {
			fRec144_perm[l62] = 0.0f;
		}
		for (int l63 = 0; l63 < 4; l63 = l63 + 1) {
			fRec145_perm[l63] = 0.0f;
		}
		for (int l64 = 0; l64 < 4; l64 = l64 + 1) {
			fRec153_perm[l64] = 0.0f;
		}
		for (int l65 = 0; l65 < 4; l65 = l65 + 1) {
			fRec154_perm[l65] = 0.0f;
		}
		for (int l66 = 0; l66 < 4; l66 = l66 + 1) {
			fRec139_perm[l66] = 0.0f;
		}
		for (int l67 = 0; l67 < 4; l67 = l67 + 1) {
			fRec140_perm[l67] = 0.0f;
		}
		for (int l68 = 0; l68 < 4; l68 = l68 + 1) {
			fRec135_perm[l68] = 0.0f;
		}
		for (int l69 = 0; l69 < 4; l69 = l69 + 1) {
			fRec136_perm[l69] = 0.0f;
		}
		for (int l70 = 0; l70 < 4; l70 = l70 + 1) {
			fRec130_perm[l70] = 0.0f;
		}
		for (int l71 = 0; l71 < 4; l71 = l71 + 1) {
			fRec131_perm[l71] = 0.0f;
		}
		for (int l72 = 0; l72 < 4; l72 = l72 + 1) {
			fRec126_perm[l72] = 0.0f;
		}
		for (int l73 = 0; l73 < 4; l73 = l73 + 1) {
			fRec127_perm[l73] = 0.0f;
		}
		for (int l74 = 0; l74 < 4; l74 = l74 + 1) {
			fRec125_perm[l74] = 0.0f;
		}
		for (int l75 = 0; l75 < 4; l75 = l75 + 1) {
			fRec124_perm[l75] = 0.0f;
		}
		for (int l76 = 0; l76 < 4; l76 = l76 + 1) {
			fRec87_perm[l76] = 0.0f;
		}
		for (int l77 = 0; l77 < 4; l77 = l77 + 1) {
			fRec88_perm[l77] = 0.0f;
		}
		for (int l78 = 0; l78 < 4; l78 = l78 + 1) {
			fRec82_perm[l78] = 0.0f;
		}
		for (int l79 = 0; l79 < 4; l79 = l79 + 1) {
			fRec83_perm[l79] = 0.0f;
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
		ui_interface->declare(&fHslider3, "unit", "dB");
		ui_interface->addHorizontalSlider("Bass", &fHslider3, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Bright", &fEntry6, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Clip Type", &fEntry2, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider6, "unit", "dB");
		ui_interface->addHorizontalSlider("Depth", &fHslider6, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Feedback", &fEntry11, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Gain", &fHslider1, FAUSTFLOAT(0.25118864f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->addNumEntry("Gate Pos", &fEntry10, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider2, "unit", "dB");
		ui_interface->addHorizontalSlider("Gate", &fHslider2, FAUSTFLOAT(-8e+01f), FAUSTFLOAT(-8e+01f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("M45", &fEntry5, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Master", &fHslider0, FAUSTFLOAT(0.5011872f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->declare(&fHslider4, "unit", "dB");
		ui_interface->addHorizontalSlider("Middle", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Pre-Shape", &fEntry0, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Pre-Shape Bite", &fEntry4, FAUSTFLOAT(3.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(6.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Pre-Shape Tight", &fEntry1, FAUSTFLOAT(-3.0f), FAUSTFLOAT(-6.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider7, "unit", "dB");
		ui_interface->addHorizontalSlider("Presence", &fHslider7, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Tight", &fEntry8, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider5, "unit", "dB");
		ui_interface->addHorizontalSlider("Treble", &fHslider5, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("WARCLAW", &fEntry9, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
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
		float fRec29_tmp[36];
		float* fRec29 = &fRec29_tmp[4];
		float fRec40_tmp[36];
		float* fRec40 = &fRec40_tmp[4];
		float fSlow2 = fConst1 * static_cast<float>(fHslider2);
		float fRec41_tmp[36];
		float* fRec41 = &fRec41_tmp[4];
		float fZec0[32];
		float fRec39_tmp[36];
		float* fRec39 = &fRec39_tmp[4];
		float fZec1[32];
		float fRec34_tmp[36];
		float* fRec34 = &fRec34_tmp[4];
		float fZec2[32];
		float fRec35_tmp[36];
		float* fRec35 = &fRec35_tmp[4];
		float fZec3[32];
		float fRec36[32];
		float fZec4[32];
		float fRec37[32];
		float fRec38[32];
		float fSlow3 = static_cast<float>(fEntry0);
		float fSlow4 = std::pow(1e+01f, 0.05f * static_cast<float>(fEntry1) * fSlow3);
		float fSlow5 = 1.4285715f * std::sqrt(fSlow4);
		float fZec5[32];
		float fRec30_tmp[36];
		float* fRec30 = &fRec30_tmp[4];
		float fZec6[32];
		float fRec31_tmp[36];
		float* fRec31 = &fRec31_tmp[4];
		float fRec32[32];
		float fZec7[32];
		float fZec8[32];
		float fRec33[32];
		int iSlow6 = static_cast<int>(static_cast<float>(fEntry2)) >= 1;
		int iSlow7 = static_cast<float>(fEntry3) > 0.5f;
		float fZec9[32];
		float fSlow8 = std::pow(1e+01f, 0.05f * fSlow3 * static_cast<float>(fEntry4));
		float fSlow9 = 1.0f - 0.35f * static_cast<float>(fEntry5);
		float fSlow10 = 0.22f * (1.2f * static_cast<float>(fEntry6) + 1.5f) * fSlow9;
		float fZec10[32];
		float fSlow11 = 0.5f * static_cast<float>(fEntry7);
		float fSlow12 = tanhf(fSlow11);
		float fZec11[32];
		float fZec12[32];
		float fRec42_tmp[36];
		float* fRec42 = &fRec42_tmp[4];
		float fZec13[32];
		float fRec43_tmp[36];
		float* fRec43 = &fRec43_tmp[4];
		float fZec14[32];
		float fRec44[32];
		int iSlow13 = static_cast<float>(fEntry8) > 0.5f;
		float fSlow14 = 0.34f * fSlow9;
		float fZec15[32];
		float fZec16[32];
		float fZec17[32];
		float fRec24_tmp[36];
		float* fRec24 = &fRec24_tmp[4];
		float fZec18[32];
		float fRec25_tmp[36];
		float* fRec25 = &fRec25_tmp[4];
		float fZec19[32];
		float fRec26[32];
		float fZec20[32];
		float fRec27[32];
		float fRec28[32];
		float fSlow15 = fConst1 * static_cast<float>(fHslider3);
		float fRec45_tmp[36];
		float* fRec45 = &fRec45_tmp[4];
		float fZec21[32];
		float fZec22[32];
		float fZec23[32];
		float fRec20_tmp[36];
		float* fRec20 = &fRec20_tmp[4];
		float fZec24[32];
		float fRec21_tmp[36];
		float* fRec21 = &fRec21_tmp[4];
		float fRec22[32];
		float fZec25[32];
		float fZec26[32];
		float fRec23[32];
		float fSlow16 = fConst1 * static_cast<float>(fHslider4);
		float fRec46_tmp[36];
		float* fRec46 = &fRec46_tmp[4];
		float fZec27[32];
		float fZec28[32];
		float fRec15_tmp[36];
		float* fRec15 = &fRec15_tmp[4];
		float fZec29[32];
		float fRec16_tmp[36];
		float* fRec16 = &fRec16_tmp[4];
		float fZec30[32];
		float fRec17[32];
		float fZec31[32];
		float fRec18[32];
		float fRec19[32];
		float fSlow17 = fConst1 * static_cast<float>(fHslider5);
		float fRec47_tmp[36];
		float* fRec47 = &fRec47_tmp[4];
		float fZec32[32];
		float fZec33[32];
		float fSlow18 = static_cast<float>(fEntry9);
		float fSlow19 = 1.9f * fSlow18 + 1.0f;
		float fZec34[32];
		float fRec11_tmp[36];
		float* fRec11 = &fRec11_tmp[4];
		float fZec35[32];
		float fRec12_tmp[36];
		float* fRec12 = &fRec12_tmp[4];
		float fRec13[32];
		float fZec36[32];
		float fZec37[32];
		float fRec14[32];
		float fZec38[32];
		float fRec72_tmp[36];
		float* fRec72 = &fRec72_tmp[4];
		float fZec39[32];
		float fRec73_tmp[36];
		float* fRec73 = &fRec73_tmp[4];
		float fZec40[32];
		float fRec74[32];
		float fZec41[32];
		float fRec75[32];
		float fRec76[32];
		float fZec42[32];
		float fRec68_tmp[36];
		float* fRec68 = &fRec68_tmp[4];
		float fZec43[32];
		float fRec69_tmp[36];
		float* fRec69 = &fRec69_tmp[4];
		float fRec70[32];
		float fZec44[32];
		float fZec45[32];
		float fRec71[32];
		float fZec46[32];
		float fZec47[32];
		float fZec48[32];
		float fRec77_tmp[36];
		float* fRec77 = &fRec77_tmp[4];
		float fZec49[32];
		float fRec78_tmp[36];
		float* fRec78 = &fRec78_tmp[4];
		float fZec50[32];
		float fRec79[32];
		float fZec51[32];
		float fZec52[32];
		float fZec53[32];
		float fRec63_tmp[36];
		float* fRec63 = &fRec63_tmp[4];
		float fZec54[32];
		float fRec64_tmp[36];
		float* fRec64 = &fRec64_tmp[4];
		float fZec55[32];
		float fRec65[32];
		float fZec56[32];
		float fRec66[32];
		float fRec67[32];
		float fZec57[32];
		float fRec59_tmp[36];
		float* fRec59 = &fRec59_tmp[4];
		float fZec58[32];
		float fRec60_tmp[36];
		float* fRec60 = &fRec60_tmp[4];
		float fRec61[32];
		float fZec59[32];
		float fZec60[32];
		float fRec62[32];
		float fZec61[32];
		float fRec54_tmp[36];
		float* fRec54 = &fRec54_tmp[4];
		float fZec62[32];
		float fRec55_tmp[36];
		float* fRec55 = &fRec55_tmp[4];
		float fZec63[32];
		float fRec56[32];
		float fZec64[32];
		float fRec57[32];
		float fRec58[32];
		float fZec65[32];
		float fRec50_tmp[36];
		float* fRec50 = &fRec50_tmp[4];
		float fZec66[32];
		float fRec51_tmp[36];
		float* fRec51 = &fRec51_tmp[4];
		float fRec52[32];
		float fZec67[32];
		float fZec68[32];
		float fRec53[32];
		float fSlow20 = std::pow(1e+01f, 0.2f * fSlow18);
		float fZec69[32];
		float fZec70[32];
		float fSlow21 = 1.0f - 0.22f * fSlow18;
		float fRec49_tmp[36];
		float* fRec49 = &fRec49_tmp[4];
		float fRec48_tmp[36];
		float* fRec48 = &fRec48_tmp[4];
		int iSlow22 = static_cast<int>(static_cast<float>(fEntry10));
		float fZec71[32];
		float fZec72[32];
		float fRec6_tmp[36];
		float* fRec6 = &fRec6_tmp[4];
		float fZec73[32];
		float fRec7_tmp[36];
		float* fRec7 = &fRec7_tmp[4];
		float fZec74[32];
		float fRec8[32];
		float fZec75[32];
		float fRec9[32];
		float fRec10[32];
		float fSlow23 = fConst1 * static_cast<float>(fHslider6);
		float fRec80_tmp[36];
		float* fRec80 = &fRec80_tmp[4];
		float fSlow24 = static_cast<float>(fEntry11);
		float fSlow25 = 0.05f * (1.25f - 0.35f * fSlow24);
		float fZec76[32];
		float fZec77[32];
		float fZec78[32];
		float fRec1_tmp[36];
		float* fRec1 = &fRec1_tmp[4];
		float fZec79[32];
		float fRec2_tmp[36];
		float* fRec2 = &fRec2_tmp[4];
		float fZec80[32];
		float fRec3[32];
		float fZec81[32];
		float fRec4[32];
		float fRec5[32];
		float fSlow26 = fConst1 * static_cast<float>(fHslider7);
		float fRec81_tmp[36];
		float* fRec81 = &fRec81_tmp[4];
		float fSlow27 = 0.05f * (0.25f * fSlow24 + 0.75f);
		float fZec82[32];
		float fZec83[32];
		float fRec120_tmp[36];
		float* fRec120 = &fRec120_tmp[4];
		float fRec119_tmp[36];
		float* fRec119 = &fRec119_tmp[4];
		float fZec84[32];
		float fZec85[32];
		float fRec114_tmp[36];
		float* fRec114 = &fRec114_tmp[4];
		float fZec86[32];
		float fRec115_tmp[36];
		float* fRec115 = &fRec115_tmp[4];
		float fZec87[32];
		float fRec116[32];
		float fZec88[32];
		float fRec117[32];
		float fRec118[32];
		float fZec89[32];
		float fRec110_tmp[36];
		float* fRec110 = &fRec110_tmp[4];
		float fZec90[32];
		float fRec111_tmp[36];
		float* fRec111 = &fRec111_tmp[4];
		float fRec112[32];
		float fZec91[32];
		float fZec92[32];
		float fRec113[32];
		float fZec93[32];
		float fZec94[32];
		float fZec95[32];
		float fRec121_tmp[36];
		float* fRec121 = &fRec121_tmp[4];
		float fZec96[32];
		float fRec122_tmp[36];
		float* fRec122 = &fRec122_tmp[4];
		float fZec97[32];
		float fRec123[32];
		float fZec98[32];
		float fZec99[32];
		float fZec100[32];
		float fRec105_tmp[36];
		float* fRec105 = &fRec105_tmp[4];
		float fZec101[32];
		float fRec106_tmp[36];
		float* fRec106 = &fRec106_tmp[4];
		float fZec102[32];
		float fRec107[32];
		float fZec103[32];
		float fRec108[32];
		float fRec109[32];
		float fZec104[32];
		float fRec101_tmp[36];
		float* fRec101 = &fRec101_tmp[4];
		float fZec105[32];
		float fRec102_tmp[36];
		float* fRec102 = &fRec102_tmp[4];
		float fRec103[32];
		float fZec106[32];
		float fZec107[32];
		float fRec104[32];
		float fZec108[32];
		float fRec96_tmp[36];
		float* fRec96 = &fRec96_tmp[4];
		float fZec109[32];
		float fRec97_tmp[36];
		float* fRec97 = &fRec97_tmp[4];
		float fZec110[32];
		float fRec98[32];
		float fZec111[32];
		float fRec99[32];
		float fRec100[32];
		float fZec112[32];
		float fRec92_tmp[36];
		float* fRec92 = &fRec92_tmp[4];
		float fZec113[32];
		float fRec93_tmp[36];
		float* fRec93 = &fRec93_tmp[4];
		float fRec94[32];
		float fZec114[32];
		float fZec115[32];
		float fRec95[32];
		float fZec116[32];
		float fRec148_tmp[36];
		float* fRec148 = &fRec148_tmp[4];
		float fZec117[32];
		float fRec149_tmp[36];
		float* fRec149 = &fRec149_tmp[4];
		float fZec118[32];
		float fRec150[32];
		float fZec119[32];
		float fRec151[32];
		float fRec152[32];
		float fZec120[32];
		float fRec144_tmp[36];
		float* fRec144 = &fRec144_tmp[4];
		float fZec121[32];
		float fRec145_tmp[36];
		float* fRec145 = &fRec145_tmp[4];
		float fRec146[32];
		float fZec122[32];
		float fZec123[32];
		float fRec147[32];
		float fZec124[32];
		float fZec125[32];
		float fZec126[32];
		float fRec153_tmp[36];
		float* fRec153 = &fRec153_tmp[4];
		float fZec127[32];
		float fRec154_tmp[36];
		float* fRec154 = &fRec154_tmp[4];
		float fZec128[32];
		float fRec155[32];
		float fZec129[32];
		float fZec130[32];
		float fZec131[32];
		float fRec139_tmp[36];
		float* fRec139 = &fRec139_tmp[4];
		float fZec132[32];
		float fRec140_tmp[36];
		float* fRec140 = &fRec140_tmp[4];
		float fZec133[32];
		float fRec141[32];
		float fZec134[32];
		float fRec142[32];
		float fRec143[32];
		float fZec135[32];
		float fRec135_tmp[36];
		float* fRec135 = &fRec135_tmp[4];
		float fZec136[32];
		float fRec136_tmp[36];
		float* fRec136 = &fRec136_tmp[4];
		float fRec137[32];
		float fZec137[32];
		float fZec138[32];
		float fRec138[32];
		float fZec139[32];
		float fRec130_tmp[36];
		float* fRec130 = &fRec130_tmp[4];
		float fZec140[32];
		float fRec131_tmp[36];
		float* fRec131 = &fRec131_tmp[4];
		float fZec141[32];
		float fRec132[32];
		float fZec142[32];
		float fRec133[32];
		float fRec134[32];
		float fZec143[32];
		float fRec126_tmp[36];
		float* fRec126 = &fRec126_tmp[4];
		float fZec144[32];
		float fRec127_tmp[36];
		float* fRec127 = &fRec127_tmp[4];
		float fRec128[32];
		float fZec145[32];
		float fZec146[32];
		float fRec129[32];
		float fZec147[32];
		float fZec148[32];
		float fRec125_tmp[36];
		float* fRec125 = &fRec125_tmp[4];
		float fRec124_tmp[36];
		float* fRec124 = &fRec124_tmp[4];
		float fZec149[32];
		float fZec150[32];
		float fRec87_tmp[36];
		float* fRec87 = &fRec87_tmp[4];
		float fZec151[32];
		float fRec88_tmp[36];
		float* fRec88 = &fRec88_tmp[4];
		float fZec152[32];
		float fRec89[32];
		float fZec153[32];
		float fRec90[32];
		float fRec91[32];
		float fZec154[32];
		float fRec82_tmp[36];
		float* fRec82 = &fRec82_tmp[4];
		float fZec155[32];
		float fRec83_tmp[36];
		float* fRec83 = &fRec83_tmp[4];
		float fZec156[32];
		float fRec84[32];
		float fZec157[32];
		float fRec85[32];
		float fRec86[32];
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
			for (int j2 = 0; j2 < 4; j2 = j2 + 1) {
				fRec29_tmp[j2] = fRec29_perm[j2];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec29[i] = fSlow1 + fConst2 * fRec29[i - 1];
			}
			/* Post code */
			for (int j3 = 0; j3 < 4; j3 = j3 + 1) {
				fRec29_perm[j3] = fRec29_tmp[vsize + j3];
			}
			/* Vectorizable loop 1 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec84[i] = static_cast<float>(input1[i]) * fRec29[i];
			}
			/* Recursive loop 2 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec41_tmp[j6] = fRec41_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec41[i] = fSlow2 + fConst2 * fRec41[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec41_perm[j7] = fRec41_tmp[vsize + j7];
			}
			/* Recursive loop 3 */
			/* Pre code */
			for (int j44 = 0; j44 < 4; j44 = j44 + 1) {
				fRec72_tmp[j44] = fRec72_perm[j44];
			}
			for (int j46 = 0; j46 < 4; j46 = j46 + 1) {
				fRec73_tmp[j46] = fRec73_perm[j46];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec38[i] = static_cast<float>(input0[i]) * fRec29[i] - (fConst4 * fRec72[i - 1] + fRec73[i - 1]);
				fRec72[i] = fRec72[i - 1] + fConst7 * fZec38[i];
				fZec39[i] = fRec72[i - 1] + fConst6 * fZec38[i];
				fRec73[i] = fRec73[i - 1] + fConst8 * fZec39[i];
				fZec40[i] = fConst3 * fZec39[i];
				fRec74[i] = fRec73[i - 1] + fZec40[i];
				fZec41[i] = fConst9 * fZec38[i];
				fRec75[i] = fZec41[i];
				fRec76[i] = fZec39[i];
			}
			/* Post code */
			for (int j45 = 0; j45 < 4; j45 = j45 + 1) {
				fRec72_perm[j45] = fRec72_tmp[vsize + j45];
			}
			for (int j47 = 0; j47 < 4; j47 = j47 + 1) {
				fRec73_perm[j47] = fRec73_tmp[vsize + j47];
			}
			/* Recursive loop 4 */
			/* Pre code */
			for (int j120 = 0; j120 < 4; j120 = j120 + 1) {
				fRec148_tmp[j120] = fRec148_perm[j120];
			}
			for (int j122 = 0; j122 < 4; j122 = j122 + 1) {
				fRec149_tmp[j122] = fRec149_perm[j122];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec116[i] = fZec84[i] - (fConst4 * fRec148[i - 1] + fRec149[i - 1]);
				fRec148[i] = fRec148[i - 1] + fConst7 * fZec116[i];
				fZec117[i] = fRec148[i - 1] + fConst6 * fZec116[i];
				fRec149[i] = fRec149[i - 1] + fConst8 * fZec117[i];
				fZec118[i] = fConst3 * fZec117[i];
				fRec150[i] = fRec149[i - 1] + fZec118[i];
				fZec119[i] = fConst9 * fZec116[i];
				fRec151[i] = fZec119[i];
				fRec152[i] = fZec117[i];
			}
			/* Post code */
			for (int j121 = 0; j121 < 4; j121 = j121 + 1) {
				fRec148_perm[j121] = fRec148_tmp[vsize + j121];
			}
			for (int j123 = 0; j123 < 4; j123 = j123 + 1) {
				fRec149_perm[j123] = fRec149_tmp[vsize + j123];
			}
			/* Recursive loop 5 */
			/* Pre code */
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec40_tmp[j4] = fRec40_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec40[i] = std::max<float>(0.995f * fRec40[i - 1], std::fabs(static_cast<float>(input0[i])));
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec40_perm[j5] = fRec40_tmp[vsize + j5];
			}
			/* Vectorizable loop 6 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::pow(1e+01f, 0.05f * fRec41[i]);
			}
			/* Vectorizable loop 7 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = 72.0f * fRec29[i] + 8.0f;
			}
			/* Recursive loop 8 */
			/* Pre code */
			for (int j48 = 0; j48 < 4; j48 = j48 + 1) {
				fRec68_tmp[j48] = fRec68_perm[j48];
			}
			for (int j50 = 0; j50 < 4; j50 = j50 + 1) {
				fRec69_tmp[j50] = fRec69_perm[j50];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec42[i] = fRec75[i] + fSlow4 * fRec74[i] + fSlow5 * fRec76[i] - (fConst11 * fRec68[i - 1] + fRec69[i - 1]);
				fRec68[i] = fRec68[i - 1] + fConst14 * fZec42[i];
				fZec43[i] = fRec68[i - 1] + fConst13 * fZec42[i];
				fRec69[i] = fRec69[i - 1] + fConst15 * fZec43[i];
				fRec70[i] = fZec43[i];
				fZec44[i] = fConst16 * fZec42[i];
				fZec45[i] = fConst10 * fZec43[i];
				fRec71[i] = fZec45[i] + fRec69[i - 1] + fZec44[i];
			}
			/* Post code */
			for (int j49 = 0; j49 < 4; j49 = j49 + 1) {
				fRec68_perm[j49] = fRec68_tmp[vsize + j49];
			}
			for (int j51 = 0; j51 < 4; j51 = j51 + 1) {
				fRec69_perm[j51] = fRec69_tmp[vsize + j51];
			}
			/* Recursive loop 9 */
			/* Pre code */
			for (int j88 = 0; j88 < 4; j88 = j88 + 1) {
				fRec120_tmp[j88] = fRec120_perm[j88];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec120[i] = std::max<float>(0.995f * fRec120[i - 1], std::fabs(static_cast<float>(input1[i])));
			}
			/* Post code */
			for (int j89 = 0; j89 < 4; j89 = j89 + 1) {
				fRec120_perm[j89] = fRec120_tmp[vsize + j89];
			}
			/* Recursive loop 10 */
			/* Pre code */
			for (int j124 = 0; j124 < 4; j124 = j124 + 1) {
				fRec144_tmp[j124] = fRec144_perm[j124];
			}
			for (int j126 = 0; j126 < 4; j126 = j126 + 1) {
				fRec145_tmp[j126] = fRec145_perm[j126];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec120[i] = fRec151[i] + fSlow4 * fRec150[i] + fSlow5 * fRec152[i] - (fConst11 * fRec144[i - 1] + fRec145[i - 1]);
				fRec144[i] = fRec144[i - 1] + fConst14 * fZec120[i];
				fZec121[i] = fRec144[i - 1] + fConst13 * fZec120[i];
				fRec145[i] = fRec145[i - 1] + fConst15 * fZec121[i];
				fRec146[i] = fZec121[i];
				fZec122[i] = fConst16 * fZec120[i];
				fZec123[i] = fConst10 * fZec121[i];
				fRec147[i] = fZec123[i] + fRec145[i - 1] + fZec122[i];
			}
			/* Post code */
			for (int j125 = 0; j125 < 4; j125 = j125 + 1) {
				fRec144_perm[j125] = fRec144_tmp[vsize + j125];
			}
			for (int j127 = 0; j127 < 4; j127 = j127 + 1) {
				fRec145_perm[j127] = fRec145_tmp[vsize + j127];
			}
			/* Recursive loop 11 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec39_tmp[j8] = fRec39_perm[j8];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec39[i] = fConst1 * static_cast<float>(fRec40[i] > fZec0[i]) + fConst2 * fRec39[i - 1];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec39_perm[j9] = fRec39_tmp[vsize + j9];
			}
			/* Vectorizable loop 12 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec46[i] = fSlow10 * fZec9[i] * (fRec71[i] + fSlow8 * fRec70[i]);
			}
			/* Recursive loop 13 */
			/* Pre code */
			for (int j90 = 0; j90 < 4; j90 = j90 + 1) {
				fRec119_tmp[j90] = fRec119_perm[j90];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec119[i] = fConst1 * static_cast<float>(fRec120[i] > fZec0[i]) + fConst2 * fRec119[i - 1];
			}
			/* Post code */
			for (int j91 = 0; j91 < 4; j91 = j91 + 1) {
				fRec119_perm[j91] = fRec119_tmp[vsize + j91];
			}
			/* Vectorizable loop 14 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec124[i] = fSlow10 * fZec9[i] * (fRec147[i] + fSlow8 * fRec146[i]);
			}
			/* Recursive loop 15 */
			/* Pre code */
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec34_tmp[j10] = fRec34_perm[j10];
			}
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec35_tmp[j12] = fRec35_perm[j12];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec1[i] = static_cast<float>(input0[i]) * fRec39[i] * fRec29[i] - (fConst4 * fRec34[i - 1] + fRec35[i - 1]);
				fRec34[i] = fRec34[i - 1] + fConst7 * fZec1[i];
				fZec2[i] = fRec34[i - 1] + fConst6 * fZec1[i];
				fRec35[i] = fRec35[i - 1] + fConst8 * fZec2[i];
				fZec3[i] = fConst3 * fZec2[i];
				fRec36[i] = fRec35[i - 1] + fZec3[i];
				fZec4[i] = fConst9 * fZec1[i];
				fRec37[i] = fZec4[i];
				fRec38[i] = fZec2[i];
			}
			/* Post code */
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec34_perm[j11] = fRec34_tmp[vsize + j11];
			}
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec35_perm[j13] = fRec35_tmp[vsize + j13];
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec47[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec46[i])))) * ((fZec46[i] > 0.0f) ? 1.0f : ((fZec46[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec46[i]) - fSlow12 : tanhf(fZec46[i])));
			}
			/* Recursive loop 17 */
			/* Pre code */
			for (int j92 = 0; j92 < 4; j92 = j92 + 1) {
				fRec114_tmp[j92] = fRec114_perm[j92];
			}
			for (int j94 = 0; j94 < 4; j94 = j94 + 1) {
				fRec115_tmp[j94] = fRec115_perm[j94];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec85[i] = fZec84[i] * fRec119[i] - (fConst4 * fRec114[i - 1] + fRec115[i - 1]);
				fRec114[i] = fRec114[i - 1] + fConst7 * fZec85[i];
				fZec86[i] = fRec114[i - 1] + fConst6 * fZec85[i];
				fRec115[i] = fRec115[i - 1] + fConst8 * fZec86[i];
				fZec87[i] = fConst3 * fZec86[i];
				fRec116[i] = fRec115[i - 1] + fZec87[i];
				fZec88[i] = fConst9 * fZec85[i];
				fRec117[i] = fZec88[i];
				fRec118[i] = fZec86[i];
			}
			/* Post code */
			for (int j93 = 0; j93 < 4; j93 = j93 + 1) {
				fRec114_perm[j93] = fRec114_tmp[vsize + j93];
			}
			for (int j95 = 0; j95 < 4; j95 = j95 + 1) {
				fRec115_perm[j95] = fRec115_tmp[vsize + j95];
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec125[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec124[i])))) * ((fZec124[i] > 0.0f) ? 1.0f : ((fZec124[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec124[i]) - fSlow12 : tanhf(fZec124[i])));
			}
			/* Recursive loop 19 */
			/* Pre code */
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec30_tmp[j14] = fRec30_perm[j14];
			}
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec31_tmp[j16] = fRec31_perm[j16];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec5[i] = fRec37[i] + fSlow4 * fRec36[i] + fSlow5 * fRec38[i] - (fConst11 * fRec30[i - 1] + fRec31[i - 1]);
				fRec30[i] = fRec30[i - 1] + fConst14 * fZec5[i];
				fZec6[i] = fRec30[i - 1] + fConst13 * fZec5[i];
				fRec31[i] = fRec31[i - 1] + fConst15 * fZec6[i];
				fRec32[i] = fZec6[i];
				fZec7[i] = fConst16 * fZec5[i];
				fZec8[i] = fConst10 * fZec6[i];
				fRec33[i] = fZec8[i] + fRec31[i - 1] + fZec7[i];
			}
			/* Post code */
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec30_perm[j15] = fRec30_tmp[vsize + j15];
			}
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec31_perm[j17] = fRec31_tmp[vsize + j17];
			}
			/* Recursive loop 20 */
			/* Pre code */
			for (int j52 = 0; j52 < 4; j52 = j52 + 1) {
				fRec77_tmp[j52] = fRec77_perm[j52];
			}
			for (int j54 = 0; j54 < 4; j54 = j54 + 1) {
				fRec78_tmp[j54] = fRec78_perm[j54];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec48[i] = fZec47[i] - (fConst18 * fRec77[i - 1] + fRec78[i - 1]);
				fRec77[i] = fRec77[i - 1] + fConst21 * fZec48[i];
				fZec49[i] = fRec77[i - 1] + fConst20 * fZec48[i];
				fRec78[i] = fRec78[i - 1] + fConst22 * fZec49[i];
				fZec50[i] = fConst23 * fZec48[i];
				fRec79[i] = fZec50[i];
			}
			/* Post code */
			for (int j53 = 0; j53 < 4; j53 = j53 + 1) {
				fRec77_perm[j53] = fRec77_tmp[vsize + j53];
			}
			for (int j55 = 0; j55 < 4; j55 = j55 + 1) {
				fRec78_perm[j55] = fRec78_tmp[vsize + j55];
			}
			/* Recursive loop 21 */
			/* Pre code */
			for (int j96 = 0; j96 < 4; j96 = j96 + 1) {
				fRec110_tmp[j96] = fRec110_perm[j96];
			}
			for (int j98 = 0; j98 < 4; j98 = j98 + 1) {
				fRec111_tmp[j98] = fRec111_perm[j98];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec89[i] = fRec117[i] + fSlow4 * fRec116[i] + fSlow5 * fRec118[i] - (fConst11 * fRec110[i - 1] + fRec111[i - 1]);
				fRec110[i] = fRec110[i - 1] + fConst14 * fZec89[i];
				fZec90[i] = fRec110[i - 1] + fConst13 * fZec89[i];
				fRec111[i] = fRec111[i - 1] + fConst15 * fZec90[i];
				fRec112[i] = fZec90[i];
				fZec91[i] = fConst16 * fZec89[i];
				fZec92[i] = fConst10 * fZec90[i];
				fRec113[i] = fZec92[i] + fRec111[i - 1] + fZec91[i];
			}
			/* Post code */
			for (int j97 = 0; j97 < 4; j97 = j97 + 1) {
				fRec110_perm[j97] = fRec110_tmp[vsize + j97];
			}
			for (int j99 = 0; j99 < 4; j99 = j99 + 1) {
				fRec111_perm[j99] = fRec111_tmp[vsize + j99];
			}
			/* Recursive loop 22 */
			/* Pre code */
			for (int j128 = 0; j128 < 4; j128 = j128 + 1) {
				fRec153_tmp[j128] = fRec153_perm[j128];
			}
			for (int j130 = 0; j130 < 4; j130 = j130 + 1) {
				fRec154_tmp[j130] = fRec154_perm[j130];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec126[i] = fZec125[i] - (fConst18 * fRec153[i - 1] + fRec154[i - 1]);
				fRec153[i] = fRec153[i - 1] + fConst21 * fZec126[i];
				fZec127[i] = fRec153[i - 1] + fConst20 * fZec126[i];
				fRec154[i] = fRec154[i - 1] + fConst22 * fZec127[i];
				fZec128[i] = fConst23 * fZec126[i];
				fRec155[i] = fZec128[i];
			}
			/* Post code */
			for (int j129 = 0; j129 < 4; j129 = j129 + 1) {
				fRec153_perm[j129] = fRec153_tmp[vsize + j129];
			}
			for (int j131 = 0; j131 < 4; j131 = j131 + 1) {
				fRec154_perm[j131] = fRec154_tmp[vsize + j131];
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = fSlow10 * (fRec33[i] + fSlow8 * fRec32[i]) * fZec9[i];
			}
			/* Recursive loop 24 */
			/* Pre code */
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec45_tmp[j26] = fRec45_perm[j26];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec45[i] = fSlow15 + fConst2 * fRec45[i - 1];
			}
			/* Post code */
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec45_perm[j27] = fRec45_tmp[vsize + j27];
			}
			/* Vectorizable loop 25 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec51[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec79[i] : fZec47[i]) + 0.03f;
			}
			/* Vectorizable loop 26 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec93[i] = fSlow10 * fZec9[i] * (fRec113[i] + fSlow8 * fRec112[i]);
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec129[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec155[i] : fZec125[i]) + 0.03f;
			}
			/* Vectorizable loop 28 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec10[i])))) * ((fZec10[i] > 0.0f) ? 1.0f : ((fZec10[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec10[i]) - fSlow12 : tanhf(fZec10[i])));
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec21[i] = std::pow(1e+01f, 0.05f * fRec45[i]);
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec52[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec51[i])))) * ((fZec51[i] > 0.0f) ? 1.0f : ((fZec51[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec51[i]) - fSlow12 : tanhf(fZec51[i])));
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec94[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec93[i])))) * ((fZec93[i] > 0.0f) ? 1.0f : ((fZec93[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec93[i]) - fSlow12 : tanhf(fZec93[i])));
			}
			/* Vectorizable loop 32 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec130[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec129[i])))) * ((fZec129[i] > 0.0f) ? 1.0f : ((fZec129[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec129[i]) - fSlow12 : tanhf(fZec129[i])));
			}
			/* Recursive loop 33 */
			/* Pre code */
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec42_tmp[j18] = fRec42_perm[j18];
			}
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec43_tmp[j20] = fRec43_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = fZec11[i] - (fConst18 * fRec42[i - 1] + fRec43[i - 1]);
				fRec42[i] = fRec42[i - 1] + fConst21 * fZec12[i];
				fZec13[i] = fRec42[i - 1] + fConst20 * fZec12[i];
				fRec43[i] = fRec43[i - 1] + fConst22 * fZec13[i];
				fZec14[i] = fConst23 * fZec12[i];
				fRec44[i] = fZec14[i];
			}
			/* Post code */
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec42_perm[j19] = fRec42_tmp[vsize + j19];
			}
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec43_perm[j21] = fRec43_tmp[vsize + j21];
			}
			/* Vectorizable loop 34 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec22[i] = std::sqrt(fZec21[i]);
			}
			/* Recursive loop 35 */
			/* Pre code */
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec46_tmp[j32] = fRec46_perm[j32];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec46[i] = fSlow16 + fConst2 * fRec46[i - 1];
			}
			/* Post code */
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec46_perm[j33] = fRec46_tmp[vsize + j33];
			}
			/* Recursive loop 36 */
			/* Pre code */
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec47_tmp[j38] = fRec47_perm[j38];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec47[i] = fSlow17 + fConst2 * fRec47[i - 1];
			}
			/* Post code */
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec47_perm[j39] = fRec47_tmp[vsize + j39];
			}
			/* Recursive loop 37 */
			/* Pre code */
			for (int j56 = 0; j56 < 4; j56 = j56 + 1) {
				fRec63_tmp[j56] = fRec63_perm[j56];
			}
			for (int j58 = 0; j58 < 4; j58 = j58 + 1) {
				fRec64_tmp[j58] = fRec64_perm[j58];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec53[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec52[i])))) * ((fZec52[i] > 0.0f) ? 1.0f : ((fZec52[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec52[i]) - fSlow12 : tanhf(fZec52[i]))) - (fConst25 * fRec63[i - 1] + fRec64[i - 1]);
				fRec63[i] = fRec63[i - 1] + fConst28 * fZec53[i];
				fZec54[i] = fRec63[i - 1] + fConst27 * fZec53[i];
				fRec64[i] = fRec64[i - 1] + fConst29 * fZec54[i];
				fZec55[i] = fConst24 * fZec54[i];
				fRec65[i] = fRec64[i - 1] + fZec55[i];
				fZec56[i] = fConst30 * fZec53[i];
				fRec66[i] = fZec56[i];
				fRec67[i] = fZec54[i];
			}
			/* Post code */
			for (int j57 = 0; j57 < 4; j57 = j57 + 1) {
				fRec63_perm[j57] = fRec63_tmp[vsize + j57];
			}
			for (int j59 = 0; j59 < 4; j59 = j59 + 1) {
				fRec64_perm[j59] = fRec64_tmp[vsize + j59];
			}
			/* Recursive loop 38 */
			/* Pre code */
			for (int j100 = 0; j100 < 4; j100 = j100 + 1) {
				fRec121_tmp[j100] = fRec121_perm[j100];
			}
			for (int j102 = 0; j102 < 4; j102 = j102 + 1) {
				fRec122_tmp[j102] = fRec122_perm[j102];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec95[i] = fZec94[i] - (fConst18 * fRec121[i - 1] + fRec122[i - 1]);
				fRec121[i] = fRec121[i - 1] + fConst21 * fZec95[i];
				fZec96[i] = fRec121[i - 1] + fConst20 * fZec95[i];
				fRec122[i] = fRec122[i - 1] + fConst22 * fZec96[i];
				fZec97[i] = fConst23 * fZec95[i];
				fRec123[i] = fZec97[i];
			}
			/* Post code */
			for (int j101 = 0; j101 < 4; j101 = j101 + 1) {
				fRec121_perm[j101] = fRec121_tmp[vsize + j101];
			}
			for (int j103 = 0; j103 < 4; j103 = j103 + 1) {
				fRec122_perm[j103] = fRec122_tmp[vsize + j103];
			}
			/* Recursive loop 39 */
			/* Pre code */
			for (int j132 = 0; j132 < 4; j132 = j132 + 1) {
				fRec139_tmp[j132] = fRec139_perm[j132];
			}
			for (int j134 = 0; j134 < 4; j134 = j134 + 1) {
				fRec140_tmp[j134] = fRec140_perm[j134];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec131[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec130[i])))) * ((fZec130[i] > 0.0f) ? 1.0f : ((fZec130[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec130[i]) - fSlow12 : tanhf(fZec130[i]))) - (fConst25 * fRec139[i - 1] + fRec140[i - 1]);
				fRec139[i] = fRec139[i - 1] + fConst28 * fZec131[i];
				fZec132[i] = fRec139[i - 1] + fConst27 * fZec131[i];
				fRec140[i] = fRec140[i - 1] + fConst29 * fZec132[i];
				fZec133[i] = fConst24 * fZec132[i];
				fRec141[i] = fRec140[i - 1] + fZec133[i];
				fZec134[i] = fConst30 * fZec131[i];
				fRec142[i] = fZec134[i];
				fRec143[i] = fZec132[i];
			}
			/* Post code */
			for (int j133 = 0; j133 < 4; j133 = j133 + 1) {
				fRec139_perm[j133] = fRec139_tmp[vsize + j133];
			}
			for (int j135 = 0; j135 < 4; j135 = j135 + 1) {
				fRec140_perm[j135] = fRec140_tmp[vsize + j135];
			}
			/* Vectorizable loop 40 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec15[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec44[i] : fZec11[i]) + 0.03f;
			}
			/* Vectorizable loop 41 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec27[i] = std::pow(1e+01f, 0.05f * (fRec46[i] + -2.5f));
			}
			/* Vectorizable loop 42 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec32[i] = std::pow(1e+01f, 0.05f * fRec47[i]);
			}
			/* Recursive loop 43 */
			/* Pre code */
			for (int j60 = 0; j60 < 4; j60 = j60 + 1) {
				fRec59_tmp[j60] = fRec59_perm[j60];
			}
			for (int j62 = 0; j62 < 4; j62 = j62 + 1) {
				fRec60_tmp[j62] = fRec60_perm[j62];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec57[i] = fRec66[i] + fRec65[i] * fZec21[i] + 1.4144272f * fRec67[i] * fZec22[i] - (fConst32 * fRec59[i - 1] + fRec60[i - 1]);
				fRec59[i] = fRec59[i - 1] + fConst35 * fZec57[i];
				fZec58[i] = fRec59[i - 1] + fConst34 * fZec57[i];
				fRec60[i] = fRec60[i - 1] + fConst36 * fZec58[i];
				fRec61[i] = fZec58[i];
				fZec59[i] = fConst37 * fZec57[i];
				fZec60[i] = fConst31 * fZec58[i];
				fRec62[i] = fZec60[i] + fRec60[i - 1] + fZec59[i];
			}
			/* Post code */
			for (int j61 = 0; j61 < 4; j61 = j61 + 1) {
				fRec59_perm[j61] = fRec59_tmp[vsize + j61];
			}
			for (int j63 = 0; j63 < 4; j63 = j63 + 1) {
				fRec60_perm[j63] = fRec60_tmp[vsize + j63];
			}
			/* Vectorizable loop 44 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec98[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec123[i] : fZec94[i]) + 0.03f;
			}
			/* Recursive loop 45 */
			/* Pre code */
			for (int j136 = 0; j136 < 4; j136 = j136 + 1) {
				fRec135_tmp[j136] = fRec135_perm[j136];
			}
			for (int j138 = 0; j138 < 4; j138 = j138 + 1) {
				fRec136_tmp[j138] = fRec136_perm[j138];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec135[i] = fRec142[i] + fRec141[i] * fZec21[i] + 1.4144272f * fRec143[i] * fZec22[i] - (fConst32 * fRec135[i - 1] + fRec136[i - 1]);
				fRec135[i] = fRec135[i - 1] + fConst35 * fZec135[i];
				fZec136[i] = fRec135[i - 1] + fConst34 * fZec135[i];
				fRec136[i] = fRec136[i - 1] + fConst36 * fZec136[i];
				fRec137[i] = fZec136[i];
				fZec137[i] = fConst37 * fZec135[i];
				fZec138[i] = fConst31 * fZec136[i];
				fRec138[i] = fZec138[i] + fRec136[i - 1] + fZec137[i];
			}
			/* Post code */
			for (int j137 = 0; j137 < 4; j137 = j137 + 1) {
				fRec135_perm[j137] = fRec135_tmp[vsize + j137];
			}
			for (int j139 = 0; j139 < 4; j139 = j139 + 1) {
				fRec136_perm[j139] = fRec136_tmp[vsize + j139];
			}
			/* Vectorizable loop 46 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec16[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec15[i])))) * ((fZec15[i] > 0.0f) ? 1.0f : ((fZec15[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec15[i]) - fSlow12 : tanhf(fZec15[i])));
			}
			/* Vectorizable loop 47 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec33[i] = std::sqrt(fZec32[i]);
			}
			/* Recursive loop 48 */
			/* Pre code */
			for (int j64 = 0; j64 < 4; j64 = j64 + 1) {
				fRec54_tmp[j64] = fRec54_perm[j64];
			}
			for (int j66 = 0; j66 < 4; j66 = j66 + 1) {
				fRec55_tmp[j66] = fRec55_perm[j66];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec61[i] = fRec62[i] + fRec61[i] * fZec27[i] - (fConst39 * fRec54[i - 1] + fRec55[i - 1]);
				fRec54[i] = fRec54[i - 1] + fConst42 * fZec61[i];
				fZec62[i] = fRec54[i - 1] + fConst41 * fZec61[i];
				fRec55[i] = fRec55[i - 1] + fConst43 * fZec62[i];
				fZec63[i] = fConst38 * fZec62[i];
				fRec56[i] = fRec55[i - 1] + fZec63[i];
				fZec64[i] = fConst44 * fZec61[i];
				fRec57[i] = fZec64[i];
				fRec58[i] = fZec62[i];
			}
			/* Post code */
			for (int j65 = 0; j65 < 4; j65 = j65 + 1) {
				fRec54_perm[j65] = fRec54_tmp[vsize + j65];
			}
			for (int j67 = 0; j67 < 4; j67 = j67 + 1) {
				fRec55_perm[j67] = fRec55_tmp[vsize + j67];
			}
			/* Vectorizable loop 49 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec99[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec98[i])))) * ((fZec98[i] > 0.0f) ? 1.0f : ((fZec98[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec98[i]) - fSlow12 : tanhf(fZec98[i])));
			}
			/* Recursive loop 50 */
			/* Pre code */
			for (int j140 = 0; j140 < 4; j140 = j140 + 1) {
				fRec130_tmp[j140] = fRec130_perm[j140];
			}
			for (int j142 = 0; j142 < 4; j142 = j142 + 1) {
				fRec131_tmp[j142] = fRec131_perm[j142];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec139[i] = fRec138[i] + fRec137[i] * fZec27[i] - (fConst39 * fRec130[i - 1] + fRec131[i - 1]);
				fRec130[i] = fRec130[i - 1] + fConst42 * fZec139[i];
				fZec140[i] = fRec130[i - 1] + fConst41 * fZec139[i];
				fRec131[i] = fRec131[i - 1] + fConst43 * fZec140[i];
				fZec141[i] = fConst38 * fZec140[i];
				fRec132[i] = fRec131[i - 1] + fZec141[i];
				fZec142[i] = fConst44 * fZec139[i];
				fRec133[i] = fZec142[i];
				fRec134[i] = fZec140[i];
			}
			/* Post code */
			for (int j141 = 0; j141 < 4; j141 = j141 + 1) {
				fRec130_perm[j141] = fRec130_tmp[vsize + j141];
			}
			for (int j143 = 0; j143 < 4; j143 = j143 + 1) {
				fRec131_perm[j143] = fRec131_tmp[vsize + j143];
			}
			/* Recursive loop 51 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec24_tmp[j22] = fRec24_perm[j22];
			}
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec25_tmp[j24] = fRec25_perm[j24];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec17[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec16[i])))) * ((fZec16[i] > 0.0f) ? 1.0f : ((fZec16[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec16[i]) - fSlow12 : tanhf(fZec16[i]))) - (fConst25 * fRec24[i - 1] + fRec25[i - 1]);
				fRec24[i] = fRec24[i - 1] + fConst28 * fZec17[i];
				fZec18[i] = fRec24[i - 1] + fConst27 * fZec17[i];
				fRec25[i] = fRec25[i - 1] + fConst29 * fZec18[i];
				fZec19[i] = fConst24 * fZec18[i];
				fRec26[i] = fRec25[i - 1] + fZec19[i];
				fZec20[i] = fConst30 * fZec17[i];
				fRec27[i] = fZec20[i];
				fRec28[i] = fZec18[i];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec24_perm[j23] = fRec24_tmp[vsize + j23];
			}
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec25_perm[j25] = fRec25_tmp[vsize + j25];
			}
			/* Recursive loop 52 */
			/* Pre code */
			for (int j68 = 0; j68 < 4; j68 = j68 + 1) {
				fRec50_tmp[j68] = fRec50_perm[j68];
			}
			for (int j70 = 0; j70 < 4; j70 = j70 + 1) {
				fRec51_tmp[j70] = fRec51_perm[j70];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec65[i] = fSlow19 * (fRec56[i] + fRec57[i] * fZec32[i] + 1.4144272f * fRec58[i] * fZec33[i]) - (fConst46 * fRec50[i - 1] + fRec51[i - 1]);
				fRec50[i] = fRec50[i - 1] + fConst49 * fZec65[i];
				fZec66[i] = fRec50[i - 1] + fConst48 * fZec65[i];
				fRec51[i] = fRec51[i - 1] + fConst50 * fZec66[i];
				fRec52[i] = fZec66[i];
				fZec67[i] = fConst51 * fZec65[i];
				fZec68[i] = fConst45 * fZec66[i];
				fRec53[i] = fZec68[i] + fRec51[i - 1] + fZec67[i];
			}
			/* Post code */
			for (int j69 = 0; j69 < 4; j69 = j69 + 1) {
				fRec50_perm[j69] = fRec50_tmp[vsize + j69];
			}
			for (int j71 = 0; j71 < 4; j71 = j71 + 1) {
				fRec51_perm[j71] = fRec51_tmp[vsize + j71];
			}
			/* Recursive loop 53 */
			/* Pre code */
			for (int j104 = 0; j104 < 4; j104 = j104 + 1) {
				fRec105_tmp[j104] = fRec105_perm[j104];
			}
			for (int j106 = 0; j106 < 4; j106 = j106 + 1) {
				fRec106_tmp[j106] = fRec106_perm[j106];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec100[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec99[i])))) * ((fZec99[i] > 0.0f) ? 1.0f : ((fZec99[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec99[i]) - fSlow12 : tanhf(fZec99[i]))) - (fConst25 * fRec105[i - 1] + fRec106[i - 1]);
				fRec105[i] = fRec105[i - 1] + fConst28 * fZec100[i];
				fZec101[i] = fRec105[i - 1] + fConst27 * fZec100[i];
				fRec106[i] = fRec106[i - 1] + fConst29 * fZec101[i];
				fZec102[i] = fConst24 * fZec101[i];
				fRec107[i] = fRec106[i - 1] + fZec102[i];
				fZec103[i] = fConst30 * fZec100[i];
				fRec108[i] = fZec103[i];
				fRec109[i] = fZec101[i];
			}
			/* Post code */
			for (int j105 = 0; j105 < 4; j105 = j105 + 1) {
				fRec105_perm[j105] = fRec105_tmp[vsize + j105];
			}
			for (int j107 = 0; j107 < 4; j107 = j107 + 1) {
				fRec106_perm[j107] = fRec106_tmp[vsize + j107];
			}
			/* Recursive loop 54 */
			/* Pre code */
			for (int j144 = 0; j144 < 4; j144 = j144 + 1) {
				fRec126_tmp[j144] = fRec126_perm[j144];
			}
			for (int j146 = 0; j146 < 4; j146 = j146 + 1) {
				fRec127_tmp[j146] = fRec127_perm[j146];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec143[i] = fSlow19 * (fRec132[i] + fRec133[i] * fZec32[i] + 1.4144272f * fRec134[i] * fZec33[i]) - (fConst46 * fRec126[i - 1] + fRec127[i - 1]);
				fRec126[i] = fRec126[i - 1] + fConst49 * fZec143[i];
				fZec144[i] = fRec126[i - 1] + fConst48 * fZec143[i];
				fRec127[i] = fRec127[i - 1] + fConst50 * fZec144[i];
				fRec128[i] = fZec144[i];
				fZec145[i] = fConst51 * fZec143[i];
				fZec146[i] = fConst45 * fZec144[i];
				fRec129[i] = fZec146[i] + fRec127[i - 1] + fZec145[i];
			}
			/* Post code */
			for (int j145 = 0; j145 < 4; j145 = j145 + 1) {
				fRec126_perm[j145] = fRec126_tmp[vsize + j145];
			}
			for (int j147 = 0; j147 < 4; j147 = j147 + 1) {
				fRec127_perm[j147] = fRec127_tmp[vsize + j147];
			}
			/* Recursive loop 55 */
			/* Pre code */
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec20_tmp[j28] = fRec20_perm[j28];
			}
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec21_tmp[j30] = fRec21_perm[j30];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec23[i] = fRec27[i] + fRec26[i] * fZec21[i] + 1.4144272f * fRec28[i] * fZec22[i] - (fConst32 * fRec20[i - 1] + fRec21[i - 1]);
				fRec20[i] = fRec20[i - 1] + fConst35 * fZec23[i];
				fZec24[i] = fRec20[i - 1] + fConst34 * fZec23[i];
				fRec21[i] = fRec21[i - 1] + fConst36 * fZec24[i];
				fRec22[i] = fZec24[i];
				fZec25[i] = fConst37 * fZec23[i];
				fZec26[i] = fConst31 * fZec24[i];
				fRec23[i] = fZec26[i] + fRec21[i - 1] + fZec25[i];
			}
			/* Post code */
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec20_perm[j29] = fRec20_tmp[vsize + j29];
			}
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec21_perm[j31] = fRec21_tmp[vsize + j31];
			}
			/* Vectorizable loop 56 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec69[i] = fRec53[i] + fSlow20 * fRec52[i];
			}
			/* Recursive loop 57 */
			/* Pre code */
			for (int j108 = 0; j108 < 4; j108 = j108 + 1) {
				fRec101_tmp[j108] = fRec101_perm[j108];
			}
			for (int j110 = 0; j110 < 4; j110 = j110 + 1) {
				fRec102_tmp[j110] = fRec102_perm[j110];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec104[i] = fRec108[i] + fRec107[i] * fZec21[i] + 1.4144272f * fRec109[i] * fZec22[i] - (fConst32 * fRec101[i - 1] + fRec102[i - 1]);
				fRec101[i] = fRec101[i - 1] + fConst35 * fZec104[i];
				fZec105[i] = fRec101[i - 1] + fConst34 * fZec104[i];
				fRec102[i] = fRec102[i - 1] + fConst36 * fZec105[i];
				fRec103[i] = fZec105[i];
				fZec106[i] = fConst37 * fZec104[i];
				fZec107[i] = fConst31 * fZec105[i];
				fRec104[i] = fZec107[i] + fRec102[i - 1] + fZec106[i];
			}
			/* Post code */
			for (int j109 = 0; j109 < 4; j109 = j109 + 1) {
				fRec101_perm[j109] = fRec101_tmp[vsize + j109];
			}
			for (int j111 = 0; j111 < 4; j111 = j111 + 1) {
				fRec102_perm[j111] = fRec102_tmp[vsize + j111];
			}
			/* Vectorizable loop 58 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec147[i] = fRec129[i] + fSlow20 * fRec128[i];
			}
			/* Recursive loop 59 */
			/* Pre code */
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec15_tmp[j34] = fRec15_perm[j34];
			}
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec16_tmp[j36] = fRec16_perm[j36];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec28[i] = fRec23[i] + fRec22[i] * fZec27[i] - (fConst39 * fRec15[i - 1] + fRec16[i - 1]);
				fRec15[i] = fRec15[i - 1] + fConst42 * fZec28[i];
				fZec29[i] = fRec15[i - 1] + fConst41 * fZec28[i];
				fRec16[i] = fRec16[i - 1] + fConst43 * fZec29[i];
				fZec30[i] = fConst38 * fZec29[i];
				fRec17[i] = fRec16[i - 1] + fZec30[i];
				fZec31[i] = fConst44 * fZec28[i];
				fRec18[i] = fZec31[i];
				fRec19[i] = fZec29[i];
			}
			/* Post code */
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec15_perm[j35] = fRec15_tmp[vsize + j35];
			}
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec16_perm[j37] = fRec16_tmp[vsize + j37];
			}
			/* Vectorizable loop 60 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec70[i] = ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec69[i])))) * ((fZec69[i] > 0.0f) ? 1.0f : ((fZec69[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec69[i]) - fSlow12 : tanhf(fZec69[i])));
			}
			/* Recursive loop 61 */
			/* Pre code */
			for (int j112 = 0; j112 < 4; j112 = j112 + 1) {
				fRec96_tmp[j112] = fRec96_perm[j112];
			}
			for (int j114 = 0; j114 < 4; j114 = j114 + 1) {
				fRec97_tmp[j114] = fRec97_perm[j114];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec108[i] = fRec104[i] + fRec103[i] * fZec27[i] - (fConst39 * fRec96[i - 1] + fRec97[i - 1]);
				fRec96[i] = fRec96[i - 1] + fConst42 * fZec108[i];
				fZec109[i] = fRec96[i - 1] + fConst41 * fZec108[i];
				fRec97[i] = fRec97[i - 1] + fConst43 * fZec109[i];
				fZec110[i] = fConst38 * fZec109[i];
				fRec98[i] = fRec97[i - 1] + fZec110[i];
				fZec111[i] = fConst44 * fZec108[i];
				fRec99[i] = fZec111[i];
				fRec100[i] = fZec109[i];
			}
			/* Post code */
			for (int j113 = 0; j113 < 4; j113 = j113 + 1) {
				fRec96_perm[j113] = fRec96_tmp[vsize + j113];
			}
			for (int j115 = 0; j115 < 4; j115 = j115 + 1) {
				fRec97_perm[j115] = fRec97_tmp[vsize + j115];
			}
			/* Vectorizable loop 62 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec148[i] = ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec147[i])))) * ((fZec147[i] > 0.0f) ? 1.0f : ((fZec147[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec147[i]) - fSlow12 : tanhf(fZec147[i])));
			}
			/* Recursive loop 63 */
			/* Pre code */
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec11_tmp[j40] = fRec11_perm[j40];
			}
			for (int j42 = 0; j42 < 4; j42 = j42 + 1) {
				fRec12_tmp[j42] = fRec12_perm[j42];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec34[i] = fSlow19 * (fRec17[i] + fRec18[i] * fZec32[i] + 1.4144272f * fRec19[i] * fZec33[i]) - (fConst46 * fRec11[i - 1] + fRec12[i - 1]);
				fRec11[i] = fRec11[i - 1] + fConst49 * fZec34[i];
				fZec35[i] = fRec11[i - 1] + fConst48 * fZec34[i];
				fRec12[i] = fRec12[i - 1] + fConst50 * fZec35[i];
				fRec13[i] = fZec35[i];
				fZec36[i] = fConst51 * fZec34[i];
				fZec37[i] = fConst45 * fZec35[i];
				fRec14[i] = fZec37[i] + fRec12[i - 1] + fZec36[i];
			}
			/* Post code */
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec11_perm[j41] = fRec11_tmp[vsize + j41];
			}
			for (int j43 = 0; j43 < 4; j43 = j43 + 1) {
				fRec12_perm[j43] = fRec12_tmp[vsize + j43];
			}
			/* Recursive loop 64 */
			/* Pre code */
			for (int j72 = 0; j72 < 4; j72 = j72 + 1) {
				fRec49_tmp[j72] = fRec49_perm[j72];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec49[i] = std::max<float>(0.995f * fRec49[i - 1], std::fabs(fSlow21 * fZec70[i]));
			}
			/* Post code */
			for (int j73 = 0; j73 < 4; j73 = j73 + 1) {
				fRec49_perm[j73] = fRec49_tmp[vsize + j73];
			}
			/* Recursive loop 65 */
			/* Pre code */
			for (int j80 = 0; j80 < 4; j80 = j80 + 1) {
				fRec80_tmp[j80] = fRec80_perm[j80];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec80[i] = fSlow23 + fConst2 * fRec80[i - 1];
			}
			/* Post code */
			for (int j81 = 0; j81 < 4; j81 = j81 + 1) {
				fRec80_perm[j81] = fRec80_tmp[vsize + j81];
			}
			/* Recursive loop 66 */
			/* Pre code */
			for (int j116 = 0; j116 < 4; j116 = j116 + 1) {
				fRec92_tmp[j116] = fRec92_perm[j116];
			}
			for (int j118 = 0; j118 < 4; j118 = j118 + 1) {
				fRec93_tmp[j118] = fRec93_perm[j118];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec112[i] = fSlow19 * (fRec98[i] + fRec99[i] * fZec32[i] + 1.4144272f * fRec100[i] * fZec33[i]) - (fConst46 * fRec92[i - 1] + fRec93[i - 1]);
				fRec92[i] = fRec92[i - 1] + fConst49 * fZec112[i];
				fZec113[i] = fRec92[i - 1] + fConst48 * fZec112[i];
				fRec93[i] = fRec93[i - 1] + fConst50 * fZec113[i];
				fRec94[i] = fZec113[i];
				fZec114[i] = fConst51 * fZec112[i];
				fZec115[i] = fConst45 * fZec113[i];
				fRec95[i] = fZec115[i] + fRec93[i - 1] + fZec114[i];
			}
			/* Post code */
			for (int j117 = 0; j117 < 4; j117 = j117 + 1) {
				fRec92_perm[j117] = fRec92_tmp[vsize + j117];
			}
			for (int j119 = 0; j119 < 4; j119 = j119 + 1) {
				fRec93_perm[j119] = fRec93_tmp[vsize + j119];
			}
			/* Recursive loop 67 */
			/* Pre code */
			for (int j148 = 0; j148 < 4; j148 = j148 + 1) {
				fRec125_tmp[j148] = fRec125_perm[j148];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec125[i] = std::max<float>(0.995f * fRec125[i - 1], std::fabs(fSlow21 * fZec148[i]));
			}
			/* Post code */
			for (int j149 = 0; j149 < 4; j149 = j149 + 1) {
				fRec125_perm[j149] = fRec125_tmp[vsize + j149];
			}
			/* Recursive loop 68 */
			/* Pre code */
			for (int j74 = 0; j74 < 4; j74 = j74 + 1) {
				fRec48_tmp[j74] = fRec48_perm[j74];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec48[i] = fConst1 * static_cast<float>(fRec49[i] > fZec0[i]) + fConst2 * fRec48[i - 1];
			}
			/* Post code */
			for (int j75 = 0; j75 < 4; j75 = j75 + 1) {
				fRec48_perm[j75] = fRec48_tmp[vsize + j75];
			}
			/* Vectorizable loop 69 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec71[i] = fRec14[i] + fSlow20 * fRec13[i];
			}
			/* Vectorizable loop 70 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec76[i] = std::pow(1e+01f, fSlow25 * fRec80[i]);
			}
			/* Recursive loop 71 */
			/* Pre code */
			for (int j86 = 0; j86 < 4; j86 = j86 + 1) {
				fRec81_tmp[j86] = fRec81_perm[j86];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec81[i] = fSlow26 + fConst2 * fRec81[i - 1];
			}
			/* Post code */
			for (int j87 = 0; j87 < 4; j87 = j87 + 1) {
				fRec81_perm[j87] = fRec81_tmp[vsize + j87];
			}
			/* Recursive loop 72 */
			/* Pre code */
			for (int j150 = 0; j150 < 4; j150 = j150 + 1) {
				fRec124_tmp[j150] = fRec124_perm[j150];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec124[i] = fConst1 * static_cast<float>(fRec125[i] > fZec0[i]) + fConst2 * fRec124[i - 1];
			}
			/* Post code */
			for (int j151 = 0; j151 < 4; j151 = j151 + 1) {
				fRec124_perm[j151] = fRec124_tmp[vsize + j151];
			}
			/* Vectorizable loop 73 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec149[i] = fRec95[i] + fSlow20 * fRec94[i];
			}
			/* Recursive loop 74 */
			/* Pre code */
			for (int j76 = 0; j76 < 4; j76 = j76 + 1) {
				fRec6_tmp[j76] = fRec6_perm[j76];
			}
			for (int j78 = 0; j78 < 4; j78 = j78 + 1) {
				fRec7_tmp[j78] = fRec7_perm[j78];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec72[i] = ((iSlow22) ? fSlow21 * fRec48[i] * fZec70[i] : fSlow21 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec71[i])))) * ((fZec71[i] > 0.0f) ? 1.0f : ((fZec71[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec71[i]) - fSlow12 : tanhf(fZec71[i])))) - (fConst53 * fRec6[i - 1] + fRec7[i - 1]);
				fRec6[i] = fRec6[i - 1] + fConst56 * fZec72[i];
				fZec73[i] = fRec6[i - 1] + fConst55 * fZec72[i];
				fRec7[i] = fRec7[i - 1] + fConst57 * fZec73[i];
				fZec74[i] = fConst52 * fZec73[i];
				fRec8[i] = fRec7[i - 1] + fZec74[i];
				fZec75[i] = fConst58 * fZec72[i];
				fRec9[i] = fZec75[i];
				fRec10[i] = fZec73[i];
			}
			/* Post code */
			for (int j77 = 0; j77 < 4; j77 = j77 + 1) {
				fRec6_perm[j77] = fRec6_tmp[vsize + j77];
			}
			for (int j79 = 0; j79 < 4; j79 = j79 + 1) {
				fRec7_perm[j79] = fRec7_tmp[vsize + j79];
			}
			/* Vectorizable loop 75 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec77[i] = std::sqrt(fZec76[i]);
			}
			/* Vectorizable loop 76 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec82[i] = std::pow(1e+01f, fSlow27 * fRec81[i]);
			}
			/* Recursive loop 77 */
			/* Pre code */
			for (int j152 = 0; j152 < 4; j152 = j152 + 1) {
				fRec87_tmp[j152] = fRec87_perm[j152];
			}
			for (int j154 = 0; j154 < 4; j154 = j154 + 1) {
				fRec88_tmp[j154] = fRec88_perm[j154];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec150[i] = ((iSlow22) ? fSlow21 * fRec124[i] * fZec148[i] : fSlow21 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec149[i])))) * ((fZec149[i] > 0.0f) ? 1.0f : ((fZec149[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec149[i]) - fSlow12 : tanhf(fZec149[i])))) - (fConst53 * fRec87[i - 1] + fRec88[i - 1]);
				fRec87[i] = fRec87[i - 1] + fConst56 * fZec150[i];
				fZec151[i] = fRec87[i - 1] + fConst55 * fZec150[i];
				fRec88[i] = fRec88[i - 1] + fConst57 * fZec151[i];
				fZec152[i] = fConst52 * fZec151[i];
				fRec89[i] = fRec88[i - 1] + fZec152[i];
				fZec153[i] = fConst58 * fZec150[i];
				fRec90[i] = fZec153[i];
				fRec91[i] = fZec151[i];
			}
			/* Post code */
			for (int j153 = 0; j153 < 4; j153 = j153 + 1) {
				fRec87_perm[j153] = fRec87_tmp[vsize + j153];
			}
			for (int j155 = 0; j155 < 4; j155 = j155 + 1) {
				fRec88_perm[j155] = fRec88_tmp[vsize + j155];
			}
			/* Recursive loop 78 */
			/* Pre code */
			for (int j82 = 0; j82 < 4; j82 = j82 + 1) {
				fRec1_tmp[j82] = fRec1_perm[j82];
			}
			for (int j84 = 0; j84 < 4; j84 = j84 + 1) {
				fRec2_tmp[j84] = fRec2_perm[j84];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec78[i] = fRec9[i] + fRec8[i] * fZec76[i] + 1.25f * fRec10[i] * fZec77[i] - (fConst60 * fRec1[i - 1] + fRec2[i - 1]);
				fRec1[i] = fRec1[i - 1] + fConst63 * fZec78[i];
				fZec79[i] = fRec1[i - 1] + fConst62 * fZec78[i];
				fRec2[i] = fRec2[i - 1] + fConst64 * fZec79[i];
				fZec80[i] = fConst59 * fZec79[i];
				fRec3[i] = fRec2[i - 1] + fZec80[i];
				fZec81[i] = fConst65 * fZec78[i];
				fRec4[i] = fZec81[i];
				fRec5[i] = fZec79[i];
			}
			/* Post code */
			for (int j83 = 0; j83 < 4; j83 = j83 + 1) {
				fRec1_perm[j83] = fRec1_tmp[vsize + j83];
			}
			for (int j85 = 0; j85 < 4; j85 = j85 + 1) {
				fRec2_perm[j85] = fRec2_tmp[vsize + j85];
			}
			/* Recursive loop 79 */
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
			/* Vectorizable loop 80 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec83[i] = std::sqrt(fZec82[i]);
			}
			/* Recursive loop 81 */
			/* Pre code */
			for (int j156 = 0; j156 < 4; j156 = j156 + 1) {
				fRec82_tmp[j156] = fRec82_perm[j156];
			}
			for (int j158 = 0; j158 < 4; j158 = j158 + 1) {
				fRec83_tmp[j158] = fRec83_perm[j158];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec154[i] = fRec90[i] + fRec89[i] * fZec76[i] + 1.25f * fRec91[i] * fZec77[i] - (fConst60 * fRec82[i - 1] + fRec83[i - 1]);
				fRec82[i] = fRec82[i - 1] + fConst63 * fZec154[i];
				fZec155[i] = fRec82[i - 1] + fConst62 * fZec154[i];
				fRec83[i] = fRec83[i - 1] + fConst64 * fZec155[i];
				fZec156[i] = fConst59 * fZec155[i];
				fRec84[i] = fRec83[i - 1] + fZec156[i];
				fZec157[i] = fConst65 * fZec154[i];
				fRec85[i] = fZec157[i];
				fRec86[i] = fZec155[i];
			}
			/* Post code */
			for (int j157 = 0; j157 < 4; j157 = j157 + 1) {
				fRec82_perm[j157] = fRec82_tmp[vsize + j157];
			}
			for (int j159 = 0; j159 < 4; j159 = j159 + 1) {
				fRec83_perm[j159] = fRec83_tmp[vsize + j159];
			}
			/* Vectorizable loop 82 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec3[i] + fRec4[i] * fZec82[i] + 1.4285715f * fRec5[i] * fZec83[i]));
			}
			/* Vectorizable loop 83 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec84[i] + fRec85[i] * fZec82[i] + 1.4285715f * fRec86[i] * fZec83[i]));
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
			for (int j2 = 0; j2 < 4; j2 = j2 + 1) {
				fRec29_tmp[j2] = fRec29_perm[j2];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec29[i] = fSlow1 + fConst2 * fRec29[i - 1];
			}
			/* Post code */
			for (int j3 = 0; j3 < 4; j3 = j3 + 1) {
				fRec29_perm[j3] = fRec29_tmp[vsize + j3];
			}
			/* Vectorizable loop 1 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec84[i] = static_cast<float>(input1[i]) * fRec29[i];
			}
			/* Recursive loop 2 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec41_tmp[j6] = fRec41_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec41[i] = fSlow2 + fConst2 * fRec41[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec41_perm[j7] = fRec41_tmp[vsize + j7];
			}
			/* Recursive loop 3 */
			/* Pre code */
			for (int j44 = 0; j44 < 4; j44 = j44 + 1) {
				fRec72_tmp[j44] = fRec72_perm[j44];
			}
			for (int j46 = 0; j46 < 4; j46 = j46 + 1) {
				fRec73_tmp[j46] = fRec73_perm[j46];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec38[i] = static_cast<float>(input0[i]) * fRec29[i] - (fConst4 * fRec72[i - 1] + fRec73[i - 1]);
				fRec72[i] = fRec72[i - 1] + fConst7 * fZec38[i];
				fZec39[i] = fRec72[i - 1] + fConst6 * fZec38[i];
				fRec73[i] = fRec73[i - 1] + fConst8 * fZec39[i];
				fZec40[i] = fConst3 * fZec39[i];
				fRec74[i] = fRec73[i - 1] + fZec40[i];
				fZec41[i] = fConst9 * fZec38[i];
				fRec75[i] = fZec41[i];
				fRec76[i] = fZec39[i];
			}
			/* Post code */
			for (int j45 = 0; j45 < 4; j45 = j45 + 1) {
				fRec72_perm[j45] = fRec72_tmp[vsize + j45];
			}
			for (int j47 = 0; j47 < 4; j47 = j47 + 1) {
				fRec73_perm[j47] = fRec73_tmp[vsize + j47];
			}
			/* Recursive loop 4 */
			/* Pre code */
			for (int j120 = 0; j120 < 4; j120 = j120 + 1) {
				fRec148_tmp[j120] = fRec148_perm[j120];
			}
			for (int j122 = 0; j122 < 4; j122 = j122 + 1) {
				fRec149_tmp[j122] = fRec149_perm[j122];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec116[i] = fZec84[i] - (fConst4 * fRec148[i - 1] + fRec149[i - 1]);
				fRec148[i] = fRec148[i - 1] + fConst7 * fZec116[i];
				fZec117[i] = fRec148[i - 1] + fConst6 * fZec116[i];
				fRec149[i] = fRec149[i - 1] + fConst8 * fZec117[i];
				fZec118[i] = fConst3 * fZec117[i];
				fRec150[i] = fRec149[i - 1] + fZec118[i];
				fZec119[i] = fConst9 * fZec116[i];
				fRec151[i] = fZec119[i];
				fRec152[i] = fZec117[i];
			}
			/* Post code */
			for (int j121 = 0; j121 < 4; j121 = j121 + 1) {
				fRec148_perm[j121] = fRec148_tmp[vsize + j121];
			}
			for (int j123 = 0; j123 < 4; j123 = j123 + 1) {
				fRec149_perm[j123] = fRec149_tmp[vsize + j123];
			}
			/* Recursive loop 5 */
			/* Pre code */
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec40_tmp[j4] = fRec40_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec40[i] = std::max<float>(0.995f * fRec40[i - 1], std::fabs(static_cast<float>(input0[i])));
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec40_perm[j5] = fRec40_tmp[vsize + j5];
			}
			/* Vectorizable loop 6 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::pow(1e+01f, 0.05f * fRec41[i]);
			}
			/* Vectorizable loop 7 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = 72.0f * fRec29[i] + 8.0f;
			}
			/* Recursive loop 8 */
			/* Pre code */
			for (int j48 = 0; j48 < 4; j48 = j48 + 1) {
				fRec68_tmp[j48] = fRec68_perm[j48];
			}
			for (int j50 = 0; j50 < 4; j50 = j50 + 1) {
				fRec69_tmp[j50] = fRec69_perm[j50];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec42[i] = fRec75[i] + fSlow4 * fRec74[i] + fSlow5 * fRec76[i] - (fConst11 * fRec68[i - 1] + fRec69[i - 1]);
				fRec68[i] = fRec68[i - 1] + fConst14 * fZec42[i];
				fZec43[i] = fRec68[i - 1] + fConst13 * fZec42[i];
				fRec69[i] = fRec69[i - 1] + fConst15 * fZec43[i];
				fRec70[i] = fZec43[i];
				fZec44[i] = fConst16 * fZec42[i];
				fZec45[i] = fConst10 * fZec43[i];
				fRec71[i] = fZec45[i] + fRec69[i - 1] + fZec44[i];
			}
			/* Post code */
			for (int j49 = 0; j49 < 4; j49 = j49 + 1) {
				fRec68_perm[j49] = fRec68_tmp[vsize + j49];
			}
			for (int j51 = 0; j51 < 4; j51 = j51 + 1) {
				fRec69_perm[j51] = fRec69_tmp[vsize + j51];
			}
			/* Recursive loop 9 */
			/* Pre code */
			for (int j88 = 0; j88 < 4; j88 = j88 + 1) {
				fRec120_tmp[j88] = fRec120_perm[j88];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec120[i] = std::max<float>(0.995f * fRec120[i - 1], std::fabs(static_cast<float>(input1[i])));
			}
			/* Post code */
			for (int j89 = 0; j89 < 4; j89 = j89 + 1) {
				fRec120_perm[j89] = fRec120_tmp[vsize + j89];
			}
			/* Recursive loop 10 */
			/* Pre code */
			for (int j124 = 0; j124 < 4; j124 = j124 + 1) {
				fRec144_tmp[j124] = fRec144_perm[j124];
			}
			for (int j126 = 0; j126 < 4; j126 = j126 + 1) {
				fRec145_tmp[j126] = fRec145_perm[j126];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec120[i] = fRec151[i] + fSlow4 * fRec150[i] + fSlow5 * fRec152[i] - (fConst11 * fRec144[i - 1] + fRec145[i - 1]);
				fRec144[i] = fRec144[i - 1] + fConst14 * fZec120[i];
				fZec121[i] = fRec144[i - 1] + fConst13 * fZec120[i];
				fRec145[i] = fRec145[i - 1] + fConst15 * fZec121[i];
				fRec146[i] = fZec121[i];
				fZec122[i] = fConst16 * fZec120[i];
				fZec123[i] = fConst10 * fZec121[i];
				fRec147[i] = fZec123[i] + fRec145[i - 1] + fZec122[i];
			}
			/* Post code */
			for (int j125 = 0; j125 < 4; j125 = j125 + 1) {
				fRec144_perm[j125] = fRec144_tmp[vsize + j125];
			}
			for (int j127 = 0; j127 < 4; j127 = j127 + 1) {
				fRec145_perm[j127] = fRec145_tmp[vsize + j127];
			}
			/* Recursive loop 11 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec39_tmp[j8] = fRec39_perm[j8];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec39[i] = fConst1 * static_cast<float>(fRec40[i] > fZec0[i]) + fConst2 * fRec39[i - 1];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec39_perm[j9] = fRec39_tmp[vsize + j9];
			}
			/* Vectorizable loop 12 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec46[i] = fSlow10 * fZec9[i] * (fRec71[i] + fSlow8 * fRec70[i]);
			}
			/* Recursive loop 13 */
			/* Pre code */
			for (int j90 = 0; j90 < 4; j90 = j90 + 1) {
				fRec119_tmp[j90] = fRec119_perm[j90];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec119[i] = fConst1 * static_cast<float>(fRec120[i] > fZec0[i]) + fConst2 * fRec119[i - 1];
			}
			/* Post code */
			for (int j91 = 0; j91 < 4; j91 = j91 + 1) {
				fRec119_perm[j91] = fRec119_tmp[vsize + j91];
			}
			/* Vectorizable loop 14 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec124[i] = fSlow10 * fZec9[i] * (fRec147[i] + fSlow8 * fRec146[i]);
			}
			/* Recursive loop 15 */
			/* Pre code */
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec34_tmp[j10] = fRec34_perm[j10];
			}
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec35_tmp[j12] = fRec35_perm[j12];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec1[i] = static_cast<float>(input0[i]) * fRec39[i] * fRec29[i] - (fConst4 * fRec34[i - 1] + fRec35[i - 1]);
				fRec34[i] = fRec34[i - 1] + fConst7 * fZec1[i];
				fZec2[i] = fRec34[i - 1] + fConst6 * fZec1[i];
				fRec35[i] = fRec35[i - 1] + fConst8 * fZec2[i];
				fZec3[i] = fConst3 * fZec2[i];
				fRec36[i] = fRec35[i - 1] + fZec3[i];
				fZec4[i] = fConst9 * fZec1[i];
				fRec37[i] = fZec4[i];
				fRec38[i] = fZec2[i];
			}
			/* Post code */
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec34_perm[j11] = fRec34_tmp[vsize + j11];
			}
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec35_perm[j13] = fRec35_tmp[vsize + j13];
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec47[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec46[i])))) * ((fZec46[i] > 0.0f) ? 1.0f : ((fZec46[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec46[i]) - fSlow12 : tanhf(fZec46[i])));
			}
			/* Recursive loop 17 */
			/* Pre code */
			for (int j92 = 0; j92 < 4; j92 = j92 + 1) {
				fRec114_tmp[j92] = fRec114_perm[j92];
			}
			for (int j94 = 0; j94 < 4; j94 = j94 + 1) {
				fRec115_tmp[j94] = fRec115_perm[j94];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec85[i] = fZec84[i] * fRec119[i] - (fConst4 * fRec114[i - 1] + fRec115[i - 1]);
				fRec114[i] = fRec114[i - 1] + fConst7 * fZec85[i];
				fZec86[i] = fRec114[i - 1] + fConst6 * fZec85[i];
				fRec115[i] = fRec115[i - 1] + fConst8 * fZec86[i];
				fZec87[i] = fConst3 * fZec86[i];
				fRec116[i] = fRec115[i - 1] + fZec87[i];
				fZec88[i] = fConst9 * fZec85[i];
				fRec117[i] = fZec88[i];
				fRec118[i] = fZec86[i];
			}
			/* Post code */
			for (int j93 = 0; j93 < 4; j93 = j93 + 1) {
				fRec114_perm[j93] = fRec114_tmp[vsize + j93];
			}
			for (int j95 = 0; j95 < 4; j95 = j95 + 1) {
				fRec115_perm[j95] = fRec115_tmp[vsize + j95];
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec125[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec124[i])))) * ((fZec124[i] > 0.0f) ? 1.0f : ((fZec124[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec124[i]) - fSlow12 : tanhf(fZec124[i])));
			}
			/* Recursive loop 19 */
			/* Pre code */
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec30_tmp[j14] = fRec30_perm[j14];
			}
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec31_tmp[j16] = fRec31_perm[j16];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec5[i] = fRec37[i] + fSlow4 * fRec36[i] + fSlow5 * fRec38[i] - (fConst11 * fRec30[i - 1] + fRec31[i - 1]);
				fRec30[i] = fRec30[i - 1] + fConst14 * fZec5[i];
				fZec6[i] = fRec30[i - 1] + fConst13 * fZec5[i];
				fRec31[i] = fRec31[i - 1] + fConst15 * fZec6[i];
				fRec32[i] = fZec6[i];
				fZec7[i] = fConst16 * fZec5[i];
				fZec8[i] = fConst10 * fZec6[i];
				fRec33[i] = fZec8[i] + fRec31[i - 1] + fZec7[i];
			}
			/* Post code */
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec30_perm[j15] = fRec30_tmp[vsize + j15];
			}
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec31_perm[j17] = fRec31_tmp[vsize + j17];
			}
			/* Recursive loop 20 */
			/* Pre code */
			for (int j52 = 0; j52 < 4; j52 = j52 + 1) {
				fRec77_tmp[j52] = fRec77_perm[j52];
			}
			for (int j54 = 0; j54 < 4; j54 = j54 + 1) {
				fRec78_tmp[j54] = fRec78_perm[j54];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec48[i] = fZec47[i] - (fConst18 * fRec77[i - 1] + fRec78[i - 1]);
				fRec77[i] = fRec77[i - 1] + fConst21 * fZec48[i];
				fZec49[i] = fRec77[i - 1] + fConst20 * fZec48[i];
				fRec78[i] = fRec78[i - 1] + fConst22 * fZec49[i];
				fZec50[i] = fConst23 * fZec48[i];
				fRec79[i] = fZec50[i];
			}
			/* Post code */
			for (int j53 = 0; j53 < 4; j53 = j53 + 1) {
				fRec77_perm[j53] = fRec77_tmp[vsize + j53];
			}
			for (int j55 = 0; j55 < 4; j55 = j55 + 1) {
				fRec78_perm[j55] = fRec78_tmp[vsize + j55];
			}
			/* Recursive loop 21 */
			/* Pre code */
			for (int j96 = 0; j96 < 4; j96 = j96 + 1) {
				fRec110_tmp[j96] = fRec110_perm[j96];
			}
			for (int j98 = 0; j98 < 4; j98 = j98 + 1) {
				fRec111_tmp[j98] = fRec111_perm[j98];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec89[i] = fRec117[i] + fSlow4 * fRec116[i] + fSlow5 * fRec118[i] - (fConst11 * fRec110[i - 1] + fRec111[i - 1]);
				fRec110[i] = fRec110[i - 1] + fConst14 * fZec89[i];
				fZec90[i] = fRec110[i - 1] + fConst13 * fZec89[i];
				fRec111[i] = fRec111[i - 1] + fConst15 * fZec90[i];
				fRec112[i] = fZec90[i];
				fZec91[i] = fConst16 * fZec89[i];
				fZec92[i] = fConst10 * fZec90[i];
				fRec113[i] = fZec92[i] + fRec111[i - 1] + fZec91[i];
			}
			/* Post code */
			for (int j97 = 0; j97 < 4; j97 = j97 + 1) {
				fRec110_perm[j97] = fRec110_tmp[vsize + j97];
			}
			for (int j99 = 0; j99 < 4; j99 = j99 + 1) {
				fRec111_perm[j99] = fRec111_tmp[vsize + j99];
			}
			/* Recursive loop 22 */
			/* Pre code */
			for (int j128 = 0; j128 < 4; j128 = j128 + 1) {
				fRec153_tmp[j128] = fRec153_perm[j128];
			}
			for (int j130 = 0; j130 < 4; j130 = j130 + 1) {
				fRec154_tmp[j130] = fRec154_perm[j130];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec126[i] = fZec125[i] - (fConst18 * fRec153[i - 1] + fRec154[i - 1]);
				fRec153[i] = fRec153[i - 1] + fConst21 * fZec126[i];
				fZec127[i] = fRec153[i - 1] + fConst20 * fZec126[i];
				fRec154[i] = fRec154[i - 1] + fConst22 * fZec127[i];
				fZec128[i] = fConst23 * fZec126[i];
				fRec155[i] = fZec128[i];
			}
			/* Post code */
			for (int j129 = 0; j129 < 4; j129 = j129 + 1) {
				fRec153_perm[j129] = fRec153_tmp[vsize + j129];
			}
			for (int j131 = 0; j131 < 4; j131 = j131 + 1) {
				fRec154_perm[j131] = fRec154_tmp[vsize + j131];
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = fSlow10 * (fRec33[i] + fSlow8 * fRec32[i]) * fZec9[i];
			}
			/* Recursive loop 24 */
			/* Pre code */
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec45_tmp[j26] = fRec45_perm[j26];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec45[i] = fSlow15 + fConst2 * fRec45[i - 1];
			}
			/* Post code */
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec45_perm[j27] = fRec45_tmp[vsize + j27];
			}
			/* Vectorizable loop 25 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec51[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec79[i] : fZec47[i]) + 0.03f;
			}
			/* Vectorizable loop 26 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec93[i] = fSlow10 * fZec9[i] * (fRec113[i] + fSlow8 * fRec112[i]);
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec129[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec155[i] : fZec125[i]) + 0.03f;
			}
			/* Vectorizable loop 28 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec10[i])))) * ((fZec10[i] > 0.0f) ? 1.0f : ((fZec10[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec10[i]) - fSlow12 : tanhf(fZec10[i])));
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec21[i] = std::pow(1e+01f, 0.05f * fRec45[i]);
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec52[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec51[i])))) * ((fZec51[i] > 0.0f) ? 1.0f : ((fZec51[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec51[i]) - fSlow12 : tanhf(fZec51[i])));
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec94[i] = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec93[i])))) * ((fZec93[i] > 0.0f) ? 1.0f : ((fZec93[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec93[i]) - fSlow12 : tanhf(fZec93[i])));
			}
			/* Vectorizable loop 32 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec130[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec129[i])))) * ((fZec129[i] > 0.0f) ? 1.0f : ((fZec129[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec129[i]) - fSlow12 : tanhf(fZec129[i])));
			}
			/* Recursive loop 33 */
			/* Pre code */
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec42_tmp[j18] = fRec42_perm[j18];
			}
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec43_tmp[j20] = fRec43_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = fZec11[i] - (fConst18 * fRec42[i - 1] + fRec43[i - 1]);
				fRec42[i] = fRec42[i - 1] + fConst21 * fZec12[i];
				fZec13[i] = fRec42[i - 1] + fConst20 * fZec12[i];
				fRec43[i] = fRec43[i - 1] + fConst22 * fZec13[i];
				fZec14[i] = fConst23 * fZec12[i];
				fRec44[i] = fZec14[i];
			}
			/* Post code */
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec42_perm[j19] = fRec42_tmp[vsize + j19];
			}
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec43_perm[j21] = fRec43_tmp[vsize + j21];
			}
			/* Vectorizable loop 34 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec22[i] = std::sqrt(fZec21[i]);
			}
			/* Recursive loop 35 */
			/* Pre code */
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec46_tmp[j32] = fRec46_perm[j32];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec46[i] = fSlow16 + fConst2 * fRec46[i - 1];
			}
			/* Post code */
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec46_perm[j33] = fRec46_tmp[vsize + j33];
			}
			/* Recursive loop 36 */
			/* Pre code */
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec47_tmp[j38] = fRec47_perm[j38];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec47[i] = fSlow17 + fConst2 * fRec47[i - 1];
			}
			/* Post code */
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec47_perm[j39] = fRec47_tmp[vsize + j39];
			}
			/* Recursive loop 37 */
			/* Pre code */
			for (int j56 = 0; j56 < 4; j56 = j56 + 1) {
				fRec63_tmp[j56] = fRec63_perm[j56];
			}
			for (int j58 = 0; j58 < 4; j58 = j58 + 1) {
				fRec64_tmp[j58] = fRec64_perm[j58];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec53[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec52[i])))) * ((fZec52[i] > 0.0f) ? 1.0f : ((fZec52[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec52[i]) - fSlow12 : tanhf(fZec52[i]))) - (fConst25 * fRec63[i - 1] + fRec64[i - 1]);
				fRec63[i] = fRec63[i - 1] + fConst28 * fZec53[i];
				fZec54[i] = fRec63[i - 1] + fConst27 * fZec53[i];
				fRec64[i] = fRec64[i - 1] + fConst29 * fZec54[i];
				fZec55[i] = fConst24 * fZec54[i];
				fRec65[i] = fRec64[i - 1] + fZec55[i];
				fZec56[i] = fConst30 * fZec53[i];
				fRec66[i] = fZec56[i];
				fRec67[i] = fZec54[i];
			}
			/* Post code */
			for (int j57 = 0; j57 < 4; j57 = j57 + 1) {
				fRec63_perm[j57] = fRec63_tmp[vsize + j57];
			}
			for (int j59 = 0; j59 < 4; j59 = j59 + 1) {
				fRec64_perm[j59] = fRec64_tmp[vsize + j59];
			}
			/* Recursive loop 38 */
			/* Pre code */
			for (int j100 = 0; j100 < 4; j100 = j100 + 1) {
				fRec121_tmp[j100] = fRec121_perm[j100];
			}
			for (int j102 = 0; j102 < 4; j102 = j102 + 1) {
				fRec122_tmp[j102] = fRec122_perm[j102];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec95[i] = fZec94[i] - (fConst18 * fRec121[i - 1] + fRec122[i - 1]);
				fRec121[i] = fRec121[i - 1] + fConst21 * fZec95[i];
				fZec96[i] = fRec121[i - 1] + fConst20 * fZec95[i];
				fRec122[i] = fRec122[i - 1] + fConst22 * fZec96[i];
				fZec97[i] = fConst23 * fZec95[i];
				fRec123[i] = fZec97[i];
			}
			/* Post code */
			for (int j101 = 0; j101 < 4; j101 = j101 + 1) {
				fRec121_perm[j101] = fRec121_tmp[vsize + j101];
			}
			for (int j103 = 0; j103 < 4; j103 = j103 + 1) {
				fRec122_perm[j103] = fRec122_tmp[vsize + j103];
			}
			/* Recursive loop 39 */
			/* Pre code */
			for (int j132 = 0; j132 < 4; j132 = j132 + 1) {
				fRec139_tmp[j132] = fRec139_perm[j132];
			}
			for (int j134 = 0; j134 < 4; j134 = j134 + 1) {
				fRec140_tmp[j134] = fRec140_perm[j134];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec131[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec130[i])))) * ((fZec130[i] > 0.0f) ? 1.0f : ((fZec130[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec130[i]) - fSlow12 : tanhf(fZec130[i]))) - (fConst25 * fRec139[i - 1] + fRec140[i - 1]);
				fRec139[i] = fRec139[i - 1] + fConst28 * fZec131[i];
				fZec132[i] = fRec139[i - 1] + fConst27 * fZec131[i];
				fRec140[i] = fRec140[i - 1] + fConst29 * fZec132[i];
				fZec133[i] = fConst24 * fZec132[i];
				fRec141[i] = fRec140[i - 1] + fZec133[i];
				fZec134[i] = fConst30 * fZec131[i];
				fRec142[i] = fZec134[i];
				fRec143[i] = fZec132[i];
			}
			/* Post code */
			for (int j133 = 0; j133 < 4; j133 = j133 + 1) {
				fRec139_perm[j133] = fRec139_tmp[vsize + j133];
			}
			for (int j135 = 0; j135 < 4; j135 = j135 + 1) {
				fRec140_perm[j135] = fRec140_tmp[vsize + j135];
			}
			/* Vectorizable loop 40 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec15[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec44[i] : fZec11[i]) + 0.03f;
			}
			/* Vectorizable loop 41 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec27[i] = std::pow(1e+01f, 0.05f * (fRec46[i] + -2.5f));
			}
			/* Vectorizable loop 42 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec32[i] = std::pow(1e+01f, 0.05f * fRec47[i]);
			}
			/* Recursive loop 43 */
			/* Pre code */
			for (int j60 = 0; j60 < 4; j60 = j60 + 1) {
				fRec59_tmp[j60] = fRec59_perm[j60];
			}
			for (int j62 = 0; j62 < 4; j62 = j62 + 1) {
				fRec60_tmp[j62] = fRec60_perm[j62];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec57[i] = fRec66[i] + fRec65[i] * fZec21[i] + 1.4144272f * fRec67[i] * fZec22[i] - (fConst32 * fRec59[i - 1] + fRec60[i - 1]);
				fRec59[i] = fRec59[i - 1] + fConst35 * fZec57[i];
				fZec58[i] = fRec59[i - 1] + fConst34 * fZec57[i];
				fRec60[i] = fRec60[i - 1] + fConst36 * fZec58[i];
				fRec61[i] = fZec58[i];
				fZec59[i] = fConst37 * fZec57[i];
				fZec60[i] = fConst31 * fZec58[i];
				fRec62[i] = fZec60[i] + fRec60[i - 1] + fZec59[i];
			}
			/* Post code */
			for (int j61 = 0; j61 < 4; j61 = j61 + 1) {
				fRec59_perm[j61] = fRec59_tmp[vsize + j61];
			}
			for (int j63 = 0; j63 < 4; j63 = j63 + 1) {
				fRec60_perm[j63] = fRec60_tmp[vsize + j63];
			}
			/* Vectorizable loop 44 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec98[i] = fSlow14 * fZec9[i] * ((iSlow13) ? fRec123[i] : fZec94[i]) + 0.03f;
			}
			/* Recursive loop 45 */
			/* Pre code */
			for (int j136 = 0; j136 < 4; j136 = j136 + 1) {
				fRec135_tmp[j136] = fRec135_perm[j136];
			}
			for (int j138 = 0; j138 < 4; j138 = j138 + 1) {
				fRec136_tmp[j138] = fRec136_perm[j138];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec135[i] = fRec142[i] + fRec141[i] * fZec21[i] + 1.4144272f * fRec143[i] * fZec22[i] - (fConst32 * fRec135[i - 1] + fRec136[i - 1]);
				fRec135[i] = fRec135[i - 1] + fConst35 * fZec135[i];
				fZec136[i] = fRec135[i - 1] + fConst34 * fZec135[i];
				fRec136[i] = fRec136[i - 1] + fConst36 * fZec136[i];
				fRec137[i] = fZec136[i];
				fZec137[i] = fConst37 * fZec135[i];
				fZec138[i] = fConst31 * fZec136[i];
				fRec138[i] = fZec138[i] + fRec136[i - 1] + fZec137[i];
			}
			/* Post code */
			for (int j137 = 0; j137 < 4; j137 = j137 + 1) {
				fRec135_perm[j137] = fRec135_tmp[vsize + j137];
			}
			for (int j139 = 0; j139 < 4; j139 = j139 + 1) {
				fRec136_perm[j139] = fRec136_tmp[vsize + j139];
			}
			/* Vectorizable loop 46 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec16[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec15[i])))) * ((fZec15[i] > 0.0f) ? 1.0f : ((fZec15[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec15[i]) - fSlow12 : tanhf(fZec15[i])));
			}
			/* Vectorizable loop 47 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec33[i] = std::sqrt(fZec32[i]);
			}
			/* Recursive loop 48 */
			/* Pre code */
			for (int j64 = 0; j64 < 4; j64 = j64 + 1) {
				fRec54_tmp[j64] = fRec54_perm[j64];
			}
			for (int j66 = 0; j66 < 4; j66 = j66 + 1) {
				fRec55_tmp[j66] = fRec55_perm[j66];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec61[i] = fRec62[i] + fRec61[i] * fZec27[i] - (fConst39 * fRec54[i - 1] + fRec55[i - 1]);
				fRec54[i] = fRec54[i - 1] + fConst42 * fZec61[i];
				fZec62[i] = fRec54[i - 1] + fConst41 * fZec61[i];
				fRec55[i] = fRec55[i - 1] + fConst43 * fZec62[i];
				fZec63[i] = fConst38 * fZec62[i];
				fRec56[i] = fRec55[i - 1] + fZec63[i];
				fZec64[i] = fConst44 * fZec61[i];
				fRec57[i] = fZec64[i];
				fRec58[i] = fZec62[i];
			}
			/* Post code */
			for (int j65 = 0; j65 < 4; j65 = j65 + 1) {
				fRec54_perm[j65] = fRec54_tmp[vsize + j65];
			}
			for (int j67 = 0; j67 < 4; j67 = j67 + 1) {
				fRec55_perm[j67] = fRec55_tmp[vsize + j67];
			}
			/* Vectorizable loop 49 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec99[i] = 0.3128f * fZec9[i] * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec98[i])))) * ((fZec98[i] > 0.0f) ? 1.0f : ((fZec98[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec98[i]) - fSlow12 : tanhf(fZec98[i])));
			}
			/* Recursive loop 50 */
			/* Pre code */
			for (int j140 = 0; j140 < 4; j140 = j140 + 1) {
				fRec130_tmp[j140] = fRec130_perm[j140];
			}
			for (int j142 = 0; j142 < 4; j142 = j142 + 1) {
				fRec131_tmp[j142] = fRec131_perm[j142];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec139[i] = fRec138[i] + fRec137[i] * fZec27[i] - (fConst39 * fRec130[i - 1] + fRec131[i - 1]);
				fRec130[i] = fRec130[i - 1] + fConst42 * fZec139[i];
				fZec140[i] = fRec130[i - 1] + fConst41 * fZec139[i];
				fRec131[i] = fRec131[i - 1] + fConst43 * fZec140[i];
				fZec141[i] = fConst38 * fZec140[i];
				fRec132[i] = fRec131[i - 1] + fZec141[i];
				fZec142[i] = fConst44 * fZec139[i];
				fRec133[i] = fZec142[i];
				fRec134[i] = fZec140[i];
			}
			/* Post code */
			for (int j141 = 0; j141 < 4; j141 = j141 + 1) {
				fRec130_perm[j141] = fRec130_tmp[vsize + j141];
			}
			for (int j143 = 0; j143 < 4; j143 = j143 + 1) {
				fRec131_perm[j143] = fRec131_tmp[vsize + j143];
			}
			/* Recursive loop 51 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec24_tmp[j22] = fRec24_perm[j22];
			}
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec25_tmp[j24] = fRec25_perm[j24];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec17[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec16[i])))) * ((fZec16[i] > 0.0f) ? 1.0f : ((fZec16[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec16[i]) - fSlow12 : tanhf(fZec16[i]))) - (fConst25 * fRec24[i - 1] + fRec25[i - 1]);
				fRec24[i] = fRec24[i - 1] + fConst28 * fZec17[i];
				fZec18[i] = fRec24[i - 1] + fConst27 * fZec17[i];
				fRec25[i] = fRec25[i - 1] + fConst29 * fZec18[i];
				fZec19[i] = fConst24 * fZec18[i];
				fRec26[i] = fRec25[i - 1] + fZec19[i];
				fZec20[i] = fConst30 * fZec17[i];
				fRec27[i] = fZec20[i];
				fRec28[i] = fZec18[i];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec24_perm[j23] = fRec24_tmp[vsize + j23];
			}
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec25_perm[j25] = fRec25_tmp[vsize + j25];
			}
			/* Recursive loop 52 */
			/* Pre code */
			for (int j68 = 0; j68 < 4; j68 = j68 + 1) {
				fRec50_tmp[j68] = fRec50_perm[j68];
			}
			for (int j70 = 0; j70 < 4; j70 = j70 + 1) {
				fRec51_tmp[j70] = fRec51_perm[j70];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec65[i] = fSlow19 * (fRec56[i] + fRec57[i] * fZec32[i] + 1.4144272f * fRec58[i] * fZec33[i]) - (fConst46 * fRec50[i - 1] + fRec51[i - 1]);
				fRec50[i] = fRec50[i - 1] + fConst49 * fZec65[i];
				fZec66[i] = fRec50[i - 1] + fConst48 * fZec65[i];
				fRec51[i] = fRec51[i - 1] + fConst50 * fZec66[i];
				fRec52[i] = fZec66[i];
				fZec67[i] = fConst51 * fZec65[i];
				fZec68[i] = fConst45 * fZec66[i];
				fRec53[i] = fZec68[i] + fRec51[i - 1] + fZec67[i];
			}
			/* Post code */
			for (int j69 = 0; j69 < 4; j69 = j69 + 1) {
				fRec50_perm[j69] = fRec50_tmp[vsize + j69];
			}
			for (int j71 = 0; j71 < 4; j71 = j71 + 1) {
				fRec51_perm[j71] = fRec51_tmp[vsize + j71];
			}
			/* Recursive loop 53 */
			/* Pre code */
			for (int j104 = 0; j104 < 4; j104 = j104 + 1) {
				fRec105_tmp[j104] = fRec105_perm[j104];
			}
			for (int j106 = 0; j106 < 4; j106 = j106 + 1) {
				fRec106_tmp[j106] = fRec106_perm[j106];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec100[i] = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec99[i])))) * ((fZec99[i] > 0.0f) ? 1.0f : ((fZec99[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec99[i]) - fSlow12 : tanhf(fZec99[i]))) - (fConst25 * fRec105[i - 1] + fRec106[i - 1]);
				fRec105[i] = fRec105[i - 1] + fConst28 * fZec100[i];
				fZec101[i] = fRec105[i - 1] + fConst27 * fZec100[i];
				fRec106[i] = fRec106[i - 1] + fConst29 * fZec101[i];
				fZec102[i] = fConst24 * fZec101[i];
				fRec107[i] = fRec106[i - 1] + fZec102[i];
				fZec103[i] = fConst30 * fZec100[i];
				fRec108[i] = fZec103[i];
				fRec109[i] = fZec101[i];
			}
			/* Post code */
			for (int j105 = 0; j105 < 4; j105 = j105 + 1) {
				fRec105_perm[j105] = fRec105_tmp[vsize + j105];
			}
			for (int j107 = 0; j107 < 4; j107 = j107 + 1) {
				fRec106_perm[j107] = fRec106_tmp[vsize + j107];
			}
			/* Recursive loop 54 */
			/* Pre code */
			for (int j144 = 0; j144 < 4; j144 = j144 + 1) {
				fRec126_tmp[j144] = fRec126_perm[j144];
			}
			for (int j146 = 0; j146 < 4; j146 = j146 + 1) {
				fRec127_tmp[j146] = fRec127_perm[j146];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec143[i] = fSlow19 * (fRec132[i] + fRec133[i] * fZec32[i] + 1.4144272f * fRec134[i] * fZec33[i]) - (fConst46 * fRec126[i - 1] + fRec127[i - 1]);
				fRec126[i] = fRec126[i - 1] + fConst49 * fZec143[i];
				fZec144[i] = fRec126[i - 1] + fConst48 * fZec143[i];
				fRec127[i] = fRec127[i - 1] + fConst50 * fZec144[i];
				fRec128[i] = fZec144[i];
				fZec145[i] = fConst51 * fZec143[i];
				fZec146[i] = fConst45 * fZec144[i];
				fRec129[i] = fZec146[i] + fRec127[i - 1] + fZec145[i];
			}
			/* Post code */
			for (int j145 = 0; j145 < 4; j145 = j145 + 1) {
				fRec126_perm[j145] = fRec126_tmp[vsize + j145];
			}
			for (int j147 = 0; j147 < 4; j147 = j147 + 1) {
				fRec127_perm[j147] = fRec127_tmp[vsize + j147];
			}
			/* Recursive loop 55 */
			/* Pre code */
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec20_tmp[j28] = fRec20_perm[j28];
			}
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec21_tmp[j30] = fRec21_perm[j30];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec23[i] = fRec27[i] + fRec26[i] * fZec21[i] + 1.4144272f * fRec28[i] * fZec22[i] - (fConst32 * fRec20[i - 1] + fRec21[i - 1]);
				fRec20[i] = fRec20[i - 1] + fConst35 * fZec23[i];
				fZec24[i] = fRec20[i - 1] + fConst34 * fZec23[i];
				fRec21[i] = fRec21[i - 1] + fConst36 * fZec24[i];
				fRec22[i] = fZec24[i];
				fZec25[i] = fConst37 * fZec23[i];
				fZec26[i] = fConst31 * fZec24[i];
				fRec23[i] = fZec26[i] + fRec21[i - 1] + fZec25[i];
			}
			/* Post code */
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec20_perm[j29] = fRec20_tmp[vsize + j29];
			}
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec21_perm[j31] = fRec21_tmp[vsize + j31];
			}
			/* Vectorizable loop 56 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec69[i] = fRec53[i] + fSlow20 * fRec52[i];
			}
			/* Recursive loop 57 */
			/* Pre code */
			for (int j108 = 0; j108 < 4; j108 = j108 + 1) {
				fRec101_tmp[j108] = fRec101_perm[j108];
			}
			for (int j110 = 0; j110 < 4; j110 = j110 + 1) {
				fRec102_tmp[j110] = fRec102_perm[j110];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec104[i] = fRec108[i] + fRec107[i] * fZec21[i] + 1.4144272f * fRec109[i] * fZec22[i] - (fConst32 * fRec101[i - 1] + fRec102[i - 1]);
				fRec101[i] = fRec101[i - 1] + fConst35 * fZec104[i];
				fZec105[i] = fRec101[i - 1] + fConst34 * fZec104[i];
				fRec102[i] = fRec102[i - 1] + fConst36 * fZec105[i];
				fRec103[i] = fZec105[i];
				fZec106[i] = fConst37 * fZec104[i];
				fZec107[i] = fConst31 * fZec105[i];
				fRec104[i] = fZec107[i] + fRec102[i - 1] + fZec106[i];
			}
			/* Post code */
			for (int j109 = 0; j109 < 4; j109 = j109 + 1) {
				fRec101_perm[j109] = fRec101_tmp[vsize + j109];
			}
			for (int j111 = 0; j111 < 4; j111 = j111 + 1) {
				fRec102_perm[j111] = fRec102_tmp[vsize + j111];
			}
			/* Vectorizable loop 58 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec147[i] = fRec129[i] + fSlow20 * fRec128[i];
			}
			/* Recursive loop 59 */
			/* Pre code */
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec15_tmp[j34] = fRec15_perm[j34];
			}
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec16_tmp[j36] = fRec16_perm[j36];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec28[i] = fRec23[i] + fRec22[i] * fZec27[i] - (fConst39 * fRec15[i - 1] + fRec16[i - 1]);
				fRec15[i] = fRec15[i - 1] + fConst42 * fZec28[i];
				fZec29[i] = fRec15[i - 1] + fConst41 * fZec28[i];
				fRec16[i] = fRec16[i - 1] + fConst43 * fZec29[i];
				fZec30[i] = fConst38 * fZec29[i];
				fRec17[i] = fRec16[i - 1] + fZec30[i];
				fZec31[i] = fConst44 * fZec28[i];
				fRec18[i] = fZec31[i];
				fRec19[i] = fZec29[i];
			}
			/* Post code */
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec15_perm[j35] = fRec15_tmp[vsize + j35];
			}
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec16_perm[j37] = fRec16_tmp[vsize + j37];
			}
			/* Vectorizable loop 60 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec70[i] = ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec69[i])))) * ((fZec69[i] > 0.0f) ? 1.0f : ((fZec69[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec69[i]) - fSlow12 : tanhf(fZec69[i])));
			}
			/* Recursive loop 61 */
			/* Pre code */
			for (int j112 = 0; j112 < 4; j112 = j112 + 1) {
				fRec96_tmp[j112] = fRec96_perm[j112];
			}
			for (int j114 = 0; j114 < 4; j114 = j114 + 1) {
				fRec97_tmp[j114] = fRec97_perm[j114];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec108[i] = fRec104[i] + fRec103[i] * fZec27[i] - (fConst39 * fRec96[i - 1] + fRec97[i - 1]);
				fRec96[i] = fRec96[i - 1] + fConst42 * fZec108[i];
				fZec109[i] = fRec96[i - 1] + fConst41 * fZec108[i];
				fRec97[i] = fRec97[i - 1] + fConst43 * fZec109[i];
				fZec110[i] = fConst38 * fZec109[i];
				fRec98[i] = fRec97[i - 1] + fZec110[i];
				fZec111[i] = fConst44 * fZec108[i];
				fRec99[i] = fZec111[i];
				fRec100[i] = fZec109[i];
			}
			/* Post code */
			for (int j113 = 0; j113 < 4; j113 = j113 + 1) {
				fRec96_perm[j113] = fRec96_tmp[vsize + j113];
			}
			for (int j115 = 0; j115 < 4; j115 = j115 + 1) {
				fRec97_perm[j115] = fRec97_tmp[vsize + j115];
			}
			/* Vectorizable loop 62 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec148[i] = ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec147[i])))) * ((fZec147[i] > 0.0f) ? 1.0f : ((fZec147[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec147[i]) - fSlow12 : tanhf(fZec147[i])));
			}
			/* Recursive loop 63 */
			/* Pre code */
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec11_tmp[j40] = fRec11_perm[j40];
			}
			for (int j42 = 0; j42 < 4; j42 = j42 + 1) {
				fRec12_tmp[j42] = fRec12_perm[j42];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec34[i] = fSlow19 * (fRec17[i] + fRec18[i] * fZec32[i] + 1.4144272f * fRec19[i] * fZec33[i]) - (fConst46 * fRec11[i - 1] + fRec12[i - 1]);
				fRec11[i] = fRec11[i - 1] + fConst49 * fZec34[i];
				fZec35[i] = fRec11[i - 1] + fConst48 * fZec34[i];
				fRec12[i] = fRec12[i - 1] + fConst50 * fZec35[i];
				fRec13[i] = fZec35[i];
				fZec36[i] = fConst51 * fZec34[i];
				fZec37[i] = fConst45 * fZec35[i];
				fRec14[i] = fZec37[i] + fRec12[i - 1] + fZec36[i];
			}
			/* Post code */
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec11_perm[j41] = fRec11_tmp[vsize + j41];
			}
			for (int j43 = 0; j43 < 4; j43 = j43 + 1) {
				fRec12_perm[j43] = fRec12_tmp[vsize + j43];
			}
			/* Recursive loop 64 */
			/* Pre code */
			for (int j72 = 0; j72 < 4; j72 = j72 + 1) {
				fRec49_tmp[j72] = fRec49_perm[j72];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec49[i] = std::max<float>(0.995f * fRec49[i - 1], std::fabs(fSlow21 * fZec70[i]));
			}
			/* Post code */
			for (int j73 = 0; j73 < 4; j73 = j73 + 1) {
				fRec49_perm[j73] = fRec49_tmp[vsize + j73];
			}
			/* Recursive loop 65 */
			/* Pre code */
			for (int j80 = 0; j80 < 4; j80 = j80 + 1) {
				fRec80_tmp[j80] = fRec80_perm[j80];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec80[i] = fSlow23 + fConst2 * fRec80[i - 1];
			}
			/* Post code */
			for (int j81 = 0; j81 < 4; j81 = j81 + 1) {
				fRec80_perm[j81] = fRec80_tmp[vsize + j81];
			}
			/* Recursive loop 66 */
			/* Pre code */
			for (int j116 = 0; j116 < 4; j116 = j116 + 1) {
				fRec92_tmp[j116] = fRec92_perm[j116];
			}
			for (int j118 = 0; j118 < 4; j118 = j118 + 1) {
				fRec93_tmp[j118] = fRec93_perm[j118];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec112[i] = fSlow19 * (fRec98[i] + fRec99[i] * fZec32[i] + 1.4144272f * fRec100[i] * fZec33[i]) - (fConst46 * fRec92[i - 1] + fRec93[i - 1]);
				fRec92[i] = fRec92[i - 1] + fConst49 * fZec112[i];
				fZec113[i] = fRec92[i - 1] + fConst48 * fZec112[i];
				fRec93[i] = fRec93[i - 1] + fConst50 * fZec113[i];
				fRec94[i] = fZec113[i];
				fZec114[i] = fConst51 * fZec112[i];
				fZec115[i] = fConst45 * fZec113[i];
				fRec95[i] = fZec115[i] + fRec93[i - 1] + fZec114[i];
			}
			/* Post code */
			for (int j117 = 0; j117 < 4; j117 = j117 + 1) {
				fRec92_perm[j117] = fRec92_tmp[vsize + j117];
			}
			for (int j119 = 0; j119 < 4; j119 = j119 + 1) {
				fRec93_perm[j119] = fRec93_tmp[vsize + j119];
			}
			/* Recursive loop 67 */
			/* Pre code */
			for (int j148 = 0; j148 < 4; j148 = j148 + 1) {
				fRec125_tmp[j148] = fRec125_perm[j148];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec125[i] = std::max<float>(0.995f * fRec125[i - 1], std::fabs(fSlow21 * fZec148[i]));
			}
			/* Post code */
			for (int j149 = 0; j149 < 4; j149 = j149 + 1) {
				fRec125_perm[j149] = fRec125_tmp[vsize + j149];
			}
			/* Recursive loop 68 */
			/* Pre code */
			for (int j74 = 0; j74 < 4; j74 = j74 + 1) {
				fRec48_tmp[j74] = fRec48_perm[j74];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec48[i] = fConst1 * static_cast<float>(fRec49[i] > fZec0[i]) + fConst2 * fRec48[i - 1];
			}
			/* Post code */
			for (int j75 = 0; j75 < 4; j75 = j75 + 1) {
				fRec48_perm[j75] = fRec48_tmp[vsize + j75];
			}
			/* Vectorizable loop 69 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec71[i] = fRec14[i] + fSlow20 * fRec13[i];
			}
			/* Vectorizable loop 70 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec76[i] = std::pow(1e+01f, fSlow25 * fRec80[i]);
			}
			/* Recursive loop 71 */
			/* Pre code */
			for (int j86 = 0; j86 < 4; j86 = j86 + 1) {
				fRec81_tmp[j86] = fRec81_perm[j86];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec81[i] = fSlow26 + fConst2 * fRec81[i - 1];
			}
			/* Post code */
			for (int j87 = 0; j87 < 4; j87 = j87 + 1) {
				fRec81_perm[j87] = fRec81_tmp[vsize + j87];
			}
			/* Recursive loop 72 */
			/* Pre code */
			for (int j150 = 0; j150 < 4; j150 = j150 + 1) {
				fRec124_tmp[j150] = fRec124_perm[j150];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec124[i] = fConst1 * static_cast<float>(fRec125[i] > fZec0[i]) + fConst2 * fRec124[i - 1];
			}
			/* Post code */
			for (int j151 = 0; j151 < 4; j151 = j151 + 1) {
				fRec124_perm[j151] = fRec124_tmp[vsize + j151];
			}
			/* Vectorizable loop 73 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec149[i] = fRec95[i] + fSlow20 * fRec94[i];
			}
			/* Recursive loop 74 */
			/* Pre code */
			for (int j76 = 0; j76 < 4; j76 = j76 + 1) {
				fRec6_tmp[j76] = fRec6_perm[j76];
			}
			for (int j78 = 0; j78 < 4; j78 = j78 + 1) {
				fRec7_tmp[j78] = fRec7_perm[j78];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec72[i] = ((iSlow22) ? fSlow21 * fRec48[i] * fZec70[i] : fSlow21 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec71[i])))) * ((fZec71[i] > 0.0f) ? 1.0f : ((fZec71[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec71[i]) - fSlow12 : tanhf(fZec71[i])))) - (fConst53 * fRec6[i - 1] + fRec7[i - 1]);
				fRec6[i] = fRec6[i - 1] + fConst56 * fZec72[i];
				fZec73[i] = fRec6[i - 1] + fConst55 * fZec72[i];
				fRec7[i] = fRec7[i - 1] + fConst57 * fZec73[i];
				fZec74[i] = fConst52 * fZec73[i];
				fRec8[i] = fRec7[i - 1] + fZec74[i];
				fZec75[i] = fConst58 * fZec72[i];
				fRec9[i] = fZec75[i];
				fRec10[i] = fZec73[i];
			}
			/* Post code */
			for (int j77 = 0; j77 < 4; j77 = j77 + 1) {
				fRec6_perm[j77] = fRec6_tmp[vsize + j77];
			}
			for (int j79 = 0; j79 < 4; j79 = j79 + 1) {
				fRec7_perm[j79] = fRec7_tmp[vsize + j79];
			}
			/* Vectorizable loop 75 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec77[i] = std::sqrt(fZec76[i]);
			}
			/* Vectorizable loop 76 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec82[i] = std::pow(1e+01f, fSlow27 * fRec81[i]);
			}
			/* Recursive loop 77 */
			/* Pre code */
			for (int j152 = 0; j152 < 4; j152 = j152 + 1) {
				fRec87_tmp[j152] = fRec87_perm[j152];
			}
			for (int j154 = 0; j154 < 4; j154 = j154 + 1) {
				fRec88_tmp[j154] = fRec88_perm[j154];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec150[i] = ((iSlow22) ? fSlow21 * fRec124[i] * fZec148[i] : fSlow21 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fZec149[i])))) * ((fZec149[i] > 0.0f) ? 1.0f : ((fZec149[i] < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow11 + fZec149[i]) - fSlow12 : tanhf(fZec149[i])))) - (fConst53 * fRec87[i - 1] + fRec88[i - 1]);
				fRec87[i] = fRec87[i - 1] + fConst56 * fZec150[i];
				fZec151[i] = fRec87[i - 1] + fConst55 * fZec150[i];
				fRec88[i] = fRec88[i - 1] + fConst57 * fZec151[i];
				fZec152[i] = fConst52 * fZec151[i];
				fRec89[i] = fRec88[i - 1] + fZec152[i];
				fZec153[i] = fConst58 * fZec150[i];
				fRec90[i] = fZec153[i];
				fRec91[i] = fZec151[i];
			}
			/* Post code */
			for (int j153 = 0; j153 < 4; j153 = j153 + 1) {
				fRec87_perm[j153] = fRec87_tmp[vsize + j153];
			}
			for (int j155 = 0; j155 < 4; j155 = j155 + 1) {
				fRec88_perm[j155] = fRec88_tmp[vsize + j155];
			}
			/* Recursive loop 78 */
			/* Pre code */
			for (int j82 = 0; j82 < 4; j82 = j82 + 1) {
				fRec1_tmp[j82] = fRec1_perm[j82];
			}
			for (int j84 = 0; j84 < 4; j84 = j84 + 1) {
				fRec2_tmp[j84] = fRec2_perm[j84];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec78[i] = fRec9[i] + fRec8[i] * fZec76[i] + 1.25f * fRec10[i] * fZec77[i] - (fConst60 * fRec1[i - 1] + fRec2[i - 1]);
				fRec1[i] = fRec1[i - 1] + fConst63 * fZec78[i];
				fZec79[i] = fRec1[i - 1] + fConst62 * fZec78[i];
				fRec2[i] = fRec2[i - 1] + fConst64 * fZec79[i];
				fZec80[i] = fConst59 * fZec79[i];
				fRec3[i] = fRec2[i - 1] + fZec80[i];
				fZec81[i] = fConst65 * fZec78[i];
				fRec4[i] = fZec81[i];
				fRec5[i] = fZec79[i];
			}
			/* Post code */
			for (int j83 = 0; j83 < 4; j83 = j83 + 1) {
				fRec1_perm[j83] = fRec1_tmp[vsize + j83];
			}
			for (int j85 = 0; j85 < 4; j85 = j85 + 1) {
				fRec2_perm[j85] = fRec2_tmp[vsize + j85];
			}
			/* Recursive loop 79 */
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
			/* Vectorizable loop 80 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec83[i] = std::sqrt(fZec82[i]);
			}
			/* Recursive loop 81 */
			/* Pre code */
			for (int j156 = 0; j156 < 4; j156 = j156 + 1) {
				fRec82_tmp[j156] = fRec82_perm[j156];
			}
			for (int j158 = 0; j158 < 4; j158 = j158 + 1) {
				fRec83_tmp[j158] = fRec83_perm[j158];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec154[i] = fRec90[i] + fRec89[i] * fZec76[i] + 1.25f * fRec91[i] * fZec77[i] - (fConst60 * fRec82[i - 1] + fRec83[i - 1]);
				fRec82[i] = fRec82[i - 1] + fConst63 * fZec154[i];
				fZec155[i] = fRec82[i - 1] + fConst62 * fZec154[i];
				fRec83[i] = fRec83[i - 1] + fConst64 * fZec155[i];
				fZec156[i] = fConst59 * fZec155[i];
				fRec84[i] = fRec83[i - 1] + fZec156[i];
				fZec157[i] = fConst65 * fZec154[i];
				fRec85[i] = fZec157[i];
				fRec86[i] = fZec155[i];
			}
			/* Post code */
			for (int j157 = 0; j157 < 4; j157 = j157 + 1) {
				fRec82_perm[j157] = fRec82_tmp[vsize + j157];
			}
			for (int j159 = 0; j159 < 4; j159 = j159 + 1) {
				fRec83_perm[j159] = fRec83_tmp[vsize + j159];
			}
			/* Vectorizable loop 82 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec3[i] + fRec4[i] * fZec82[i] + 1.4285715f * fRec5[i] * fZec83[i]));
			}
			/* Vectorizable loop 83 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec84[i] + fRec85[i] * fZec82[i] + 1.4285715f * fRec86[i] * fZec83[i]));
			}
		}
	}

};

#endif
