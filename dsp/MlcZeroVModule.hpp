/* ------------------------------------------------------------
name: "mlc_zero_v"
Code generated with Faust 2.85.1 (https://faust.grame.fr)
Compilation options: -lang cpp -i -fpga-mem-th 4 -ct 1 -cn mlczerov -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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
	float fRec0[2];
	FAUSTFLOAT fEntry0;
	float fConst3;
	float fConst4;
	FAUSTFLOAT fHslider1;
	float fRec6[2];
	float fConst5;
	float fConst6;
	FAUSTFLOAT fEntry1;
	FAUSTFLOAT fEntry2;
	FAUSTFLOAT fEntry3;
	float fConst7;
	float fConst8;
	FAUSTFLOAT fHslider2;
	float fRec16[2];
	float fConst9;
	float fConst10;
	FAUSTFLOAT fHslider3;
	float fRec22[2];
	float fConst11;
	float fConst12;
	FAUSTFLOAT fHslider4;
	float fRec27[2];
	float fConst13;
	float fConst14;
	FAUSTFLOAT fEntry4;
	FAUSTFLOAT fHslider5;
	float fRec33[2];
	float fConst15;
	float fConst16;
	float fConst17;
	float fConst18;
	FAUSTFLOAT fHslider6;
	float fRec44[2];
	float fRec45[2];
	float fRec43[2];
	float fConst19;
	float fConst20;
	float fConst21;
	float fRec38[2];
	float fConst22;
	float fRec39[2];
	float fConst23;
	FAUSTFLOAT fEntry5;
	FAUSTFLOAT fEntry6;
	float fConst24;
	float fConst25;
	float fConst26;
	float fRec34[2];
	float fConst27;
	float fRec35[2];
	float fConst28;
	FAUSTFLOAT fEntry7;
	FAUSTFLOAT fEntry8;
	FAUSTFLOAT fEntry9;
	FAUSTFLOAT fEntry10;
	float fConst29;
	float fConst30;
	float fConst31;
	float fConst32;
	float fConst33;
	float fRec46[2];
	float fConst34;
	float fRec47[2];
	float fConst35;
	float fConst36;
	float fConst37;
	float fConst38;
	float fRec28[2];
	float fConst39;
	float fRec29[2];
	float fConst40;
	float fConst41;
	float fConst42;
	float fConst43;
	float fRec23[2];
	float fConst44;
	float fRec24[2];
	float fConst45;
	float fConst46;
	float fConst47;
	float fConst48;
	float fRec17[2];
	float fConst49;
	float fRec18[2];
	float fConst50;
	FAUSTFLOAT fEntry11;
	float fConst51;
	float fConst52;
	float fConst53;
	float fRec12[2];
	float fConst54;
	float fRec13[2];
	float fConst55;
	float fRec71[2];
	float fRec72[2];
	float fRec67[2];
	float fRec68[2];
	float fRec76[2];
	float fRec77[2];
	float fRec62[2];
	float fRec63[2];
	float fRec58[2];
	float fRec59[2];
	float fRec53[2];
	float fRec54[2];
	float fRec49[2];
	float fRec50[2];
	float fRec80[2];
	float fRec79[2];
	float fConst56;
	float fConst57;
	float fConst58;
	float fRec7[2];
	float fConst59;
	float fRec8[2];
	float fConst60;
	float fConst61;
	float fConst62;
	float fConst63;
	float fRec1[2];
	float fConst64;
	float fRec2[2];
	float fConst65;
	FAUSTFLOAT fHslider7;
	float fRec81[2];
	float fRec120[2];
	float fRec119[2];
	float fRec114[2];
	float fRec115[2];
	float fRec110[2];
	float fRec111[2];
	float fRec121[2];
	float fRec122[2];
	float fRec105[2];
	float fRec106[2];
	float fRec101[2];
	float fRec102[2];
	float fRec96[2];
	float fRec97[2];
	float fRec92[2];
	float fRec93[2];
	float fRec146[2];
	float fRec147[2];
	float fRec142[2];
	float fRec143[2];
	float fRec151[2];
	float fRec152[2];
	float fRec137[2];
	float fRec138[2];
	float fRec133[2];
	float fRec134[2];
	float fRec128[2];
	float fRec129[2];
	float fRec124[2];
	float fRec125[2];
	float fRec155[2];
	float fRec154[2];
	float fRec87[2];
	float fRec88[2];
	float fRec82[2];
	float fRec83[2];
	
 public:
	mlczerov() {
	}
	
	mlczerov(const mlczerov&) = default;
	
	virtual ~mlczerov() = default;
	
	mlczerov& operator=(const mlczerov&) = default;
	
	void metadata(Meta* m) { 
		m->declare("basics.lib/name", "Faust Basic Element Library");
		m->declare("basics.lib/version", "1.22.0");
		m->declare("compile_options", "-lang cpp -i -fpga-mem-th 4 -ct 1 -cn mlczerov -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
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
		fConst3 = std::tan(11309.733f / fConst0);
		fConst4 = fConst3 + 1.4285715f;
		fConst5 = std::tan(298.4513f / fConst0);
		fConst6 = fConst5 + 1.25f;
		fConst7 = std::tan(2984.513f / fConst0);
		fConst8 = fConst7 + 0.8333333f;
		fConst9 = std::tan(10681.415f / fConst0);
		fConst10 = fConst9 + 1.4144272f;
		fConst11 = std::tan(2387.6104f / fConst0);
		fConst12 = fConst11 + 1.1764706f;
		fConst13 = std::tan(376.99112f / fConst0);
		fConst14 = fConst13 + 1.4144272f;
		fConst15 = std::tan(3141.5928f / fConst0);
		fConst16 = fConst15 + 1.25f;
		fConst17 = std::tan(471.2389f / fConst0);
		fConst18 = fConst17 + 1.4285715f;
		fConst19 = fConst17 * fConst18 + 1.0f;
		fConst20 = fConst17 / fConst19;
		fConst21 = 2.0f * fConst20;
		fConst22 = 2.0f * fConst17;
		fConst23 = 1.0f / fConst19;
		fConst24 = fConst15 * fConst16 + 1.0f;
		fConst25 = fConst15 / fConst24;
		fConst26 = 2.0f * fConst25;
		fConst27 = 2.0f * fConst15;
		fConst28 = 1.0f / fConst24;
		fConst29 = std::tan(251.32741f / fConst0);
		fConst30 = fConst29 + 1.4144272f;
		fConst31 = fConst29 * fConst30 + 1.0f;
		fConst32 = fConst29 / fConst31;
		fConst33 = 2.0f * fConst32;
		fConst34 = 2.0f * fConst29;
		fConst35 = 1.0f / fConst31;
		fConst36 = fConst13 * fConst14 + 1.0f;
		fConst37 = fConst13 / fConst36;
		fConst38 = 2.0f * fConst37;
		fConst39 = 2.0f * fConst13;
		fConst40 = 1.0f / fConst36;
		fConst41 = fConst11 * fConst12 + 1.0f;
		fConst42 = fConst11 / fConst41;
		fConst43 = 2.0f * fConst42;
		fConst44 = 2.0f * fConst11;
		fConst45 = 1.0f / fConst41;
		fConst46 = fConst9 * fConst10 + 1.0f;
		fConst47 = fConst9 / fConst46;
		fConst48 = 2.0f * fConst47;
		fConst49 = 2.0f * fConst9;
		fConst50 = 1.0f / fConst46;
		fConst51 = fConst7 * fConst8 + 1.0f;
		fConst52 = fConst7 / fConst51;
		fConst53 = 2.0f * fConst52;
		fConst54 = 2.0f * fConst7;
		fConst55 = 1.0f / fConst51;
		fConst56 = fConst5 * fConst6 + 1.0f;
		fConst57 = fConst5 / fConst56;
		fConst58 = 2.0f * fConst57;
		fConst59 = 2.0f * fConst5;
		fConst60 = 1.0f / fConst56;
		fConst61 = fConst3 * fConst4 + 1.0f;
		fConst62 = fConst3 / fConst61;
		fConst63 = 2.0f * fConst62;
		fConst64 = 2.0f * fConst3;
		fConst65 = 1.0f / fConst61;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry0 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider1 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry1 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry2 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry3 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider2 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry4 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider5 = static_cast<FAUSTFLOAT>(0.25118864f);
		fHslider6 = static_cast<FAUSTFLOAT>(-8e+01f);
		fEntry5 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry6 = static_cast<FAUSTFLOAT>(-3.0f);
		fEntry7 = static_cast<FAUSTFLOAT>(3.0f);
		fEntry8 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry9 = static_cast<FAUSTFLOAT>(1.0f);
		fEntry10 = static_cast<FAUSTFLOAT>(0.5f);
		fEntry11 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider7 = static_cast<FAUSTFLOAT>(0.5011872f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 2; l0 = l0 + 1) {
			fRec0[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 2; l1 = l1 + 1) {
			fRec6[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 2; l2 = l2 + 1) {
			fRec16[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 2; l3 = l3 + 1) {
			fRec22[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 2; l4 = l4 + 1) {
			fRec27[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 2; l5 = l5 + 1) {
			fRec33[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 2; l6 = l6 + 1) {
			fRec44[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 2; l7 = l7 + 1) {
			fRec45[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 2; l8 = l8 + 1) {
			fRec43[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 2; l9 = l9 + 1) {
			fRec38[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 2; l10 = l10 + 1) {
			fRec39[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 2; l11 = l11 + 1) {
			fRec34[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 2; l12 = l12 + 1) {
			fRec35[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 2; l13 = l13 + 1) {
			fRec46[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 2; l14 = l14 + 1) {
			fRec47[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 2; l15 = l15 + 1) {
			fRec28[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 2; l16 = l16 + 1) {
			fRec29[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 2; l17 = l17 + 1) {
			fRec23[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 2; l18 = l18 + 1) {
			fRec24[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 2; l19 = l19 + 1) {
			fRec17[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 2; l20 = l20 + 1) {
			fRec18[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 2; l21 = l21 + 1) {
			fRec12[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 2; l22 = l22 + 1) {
			fRec13[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 2; l23 = l23 + 1) {
			fRec71[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 2; l24 = l24 + 1) {
			fRec72[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 2; l25 = l25 + 1) {
			fRec67[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 2; l26 = l26 + 1) {
			fRec68[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 2; l27 = l27 + 1) {
			fRec76[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 2; l28 = l28 + 1) {
			fRec77[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 2; l29 = l29 + 1) {
			fRec62[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 2; l30 = l30 + 1) {
			fRec63[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 2; l31 = l31 + 1) {
			fRec58[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 2; l32 = l32 + 1) {
			fRec59[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 2; l33 = l33 + 1) {
			fRec53[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 2; l34 = l34 + 1) {
			fRec54[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 2; l35 = l35 + 1) {
			fRec49[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 2; l36 = l36 + 1) {
			fRec50[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 2; l37 = l37 + 1) {
			fRec80[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 2; l38 = l38 + 1) {
			fRec79[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 2; l39 = l39 + 1) {
			fRec7[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 2; l40 = l40 + 1) {
			fRec8[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 2; l41 = l41 + 1) {
			fRec1[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 2; l42 = l42 + 1) {
			fRec2[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 2; l43 = l43 + 1) {
			fRec81[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 2; l44 = l44 + 1) {
			fRec120[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 2; l45 = l45 + 1) {
			fRec119[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 2; l46 = l46 + 1) {
			fRec114[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 2; l47 = l47 + 1) {
			fRec115[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 2; l48 = l48 + 1) {
			fRec110[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 2; l49 = l49 + 1) {
			fRec111[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 2; l50 = l50 + 1) {
			fRec121[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 2; l51 = l51 + 1) {
			fRec122[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 2; l52 = l52 + 1) {
			fRec105[l52] = 0.0f;
		}
		for (int l53 = 0; l53 < 2; l53 = l53 + 1) {
			fRec106[l53] = 0.0f;
		}
		for (int l54 = 0; l54 < 2; l54 = l54 + 1) {
			fRec101[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 2; l55 = l55 + 1) {
			fRec102[l55] = 0.0f;
		}
		for (int l56 = 0; l56 < 2; l56 = l56 + 1) {
			fRec96[l56] = 0.0f;
		}
		for (int l57 = 0; l57 < 2; l57 = l57 + 1) {
			fRec97[l57] = 0.0f;
		}
		for (int l58 = 0; l58 < 2; l58 = l58 + 1) {
			fRec92[l58] = 0.0f;
		}
		for (int l59 = 0; l59 < 2; l59 = l59 + 1) {
			fRec93[l59] = 0.0f;
		}
		for (int l60 = 0; l60 < 2; l60 = l60 + 1) {
			fRec146[l60] = 0.0f;
		}
		for (int l61 = 0; l61 < 2; l61 = l61 + 1) {
			fRec147[l61] = 0.0f;
		}
		for (int l62 = 0; l62 < 2; l62 = l62 + 1) {
			fRec142[l62] = 0.0f;
		}
		for (int l63 = 0; l63 < 2; l63 = l63 + 1) {
			fRec143[l63] = 0.0f;
		}
		for (int l64 = 0; l64 < 2; l64 = l64 + 1) {
			fRec151[l64] = 0.0f;
		}
		for (int l65 = 0; l65 < 2; l65 = l65 + 1) {
			fRec152[l65] = 0.0f;
		}
		for (int l66 = 0; l66 < 2; l66 = l66 + 1) {
			fRec137[l66] = 0.0f;
		}
		for (int l67 = 0; l67 < 2; l67 = l67 + 1) {
			fRec138[l67] = 0.0f;
		}
		for (int l68 = 0; l68 < 2; l68 = l68 + 1) {
			fRec133[l68] = 0.0f;
		}
		for (int l69 = 0; l69 < 2; l69 = l69 + 1) {
			fRec134[l69] = 0.0f;
		}
		for (int l70 = 0; l70 < 2; l70 = l70 + 1) {
			fRec128[l70] = 0.0f;
		}
		for (int l71 = 0; l71 < 2; l71 = l71 + 1) {
			fRec129[l71] = 0.0f;
		}
		for (int l72 = 0; l72 < 2; l72 = l72 + 1) {
			fRec124[l72] = 0.0f;
		}
		for (int l73 = 0; l73 < 2; l73 = l73 + 1) {
			fRec125[l73] = 0.0f;
		}
		for (int l74 = 0; l74 < 2; l74 = l74 + 1) {
			fRec155[l74] = 0.0f;
		}
		for (int l75 = 0; l75 < 2; l75 = l75 + 1) {
			fRec154[l75] = 0.0f;
		}
		for (int l76 = 0; l76 < 2; l76 = l76 + 1) {
			fRec87[l76] = 0.0f;
		}
		for (int l77 = 0; l77 < 2; l77 = l77 + 1) {
			fRec88[l77] = 0.0f;
		}
		for (int l78 = 0; l78 < 2; l78 = l78 + 1) {
			fRec82[l78] = 0.0f;
		}
		for (int l79 = 0; l79 < 2; l79 = l79 + 1) {
			fRec83[l79] = 0.0f;
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
		ui_interface->addNumEntry("Asymmetry", &fEntry10, FAUSTFLOAT(0.5f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->addNumEntry("Asymmetry Enable", &fEntry3, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider4, "unit", "dB");
		ui_interface->addHorizontalSlider("Bass", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Bright", &fEntry9, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Clip Type", &fEntry2, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider1, "unit", "dB");
		ui_interface->addHorizontalSlider("Depth", &fHslider1, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Feedback", &fEntry0, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Gain", &fHslider5, FAUSTFLOAT(0.25118864f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->addNumEntry("Gate Pos", &fEntry1, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider6, "unit", "dB");
		ui_interface->addHorizontalSlider("Gate", &fHslider6, FAUSTFLOAT(-8e+01f), FAUSTFLOAT(-8e+01f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("M45", &fEntry8, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Master", &fHslider7, FAUSTFLOAT(0.5011872f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->declare(&fHslider3, "unit", "dB");
		ui_interface->addHorizontalSlider("Middle", &fHslider3, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Pre-Shape", &fEntry5, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Pre-Shape Bite", &fEntry7, FAUSTFLOAT(3.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(6.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Pre-Shape Tight", &fEntry6, FAUSTFLOAT(-3.0f), FAUSTFLOAT(-6.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider0, "unit", "dB");
		ui_interface->addHorizontalSlider("Presence", &fHslider0, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Tight", &fEntry4, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider2, "unit", "dB");
		ui_interface->addHorizontalSlider("Treble", &fHslider2, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("WARCLAW", &fEntry11, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** RESTRICT inputs, FAUSTFLOAT** RESTRICT outputs) {
		FAUSTFLOAT* input0 = inputs[0];
		FAUSTFLOAT* input1 = inputs[1];
		FAUSTFLOAT* output0 = outputs[0];
		FAUSTFLOAT* output1 = outputs[1];
		float fSlow0 = fConst1 * static_cast<float>(fHslider0);
		float fSlow1 = static_cast<float>(fEntry0);
		float fSlow2 = 0.05f * (0.25f * fSlow1 + 0.75f);
		float fSlow3 = fConst1 * static_cast<float>(fHslider1);
		float fSlow4 = 0.05f * (1.25f - 0.35f * fSlow1);
		int iSlow5 = static_cast<int>(static_cast<float>(fEntry1));
		int iSlow6 = static_cast<int>(static_cast<float>(fEntry2)) >= 1;
		int iSlow7 = static_cast<float>(fEntry3) > 0.5f;
		float fSlow8 = fConst1 * static_cast<float>(fHslider2);
		float fSlow9 = fConst1 * static_cast<float>(fHslider3);
		float fSlow10 = fConst1 * static_cast<float>(fHslider4);
		int iSlow11 = static_cast<float>(fEntry4) > 0.5f;
		float fSlow12 = fConst1 * static_cast<float>(fHslider5);
		float fSlow13 = fConst1 * static_cast<float>(fHslider6);
		float fSlow14 = static_cast<float>(fEntry5);
		float fSlow15 = std::pow(1e+01f, 0.05f * static_cast<float>(fEntry6) * fSlow14);
		float fSlow16 = 1.4285715f * std::sqrt(fSlow15);
		float fSlow17 = std::pow(1e+01f, 0.05f * fSlow14 * static_cast<float>(fEntry7));
		float fSlow18 = 1.0f - 0.35f * static_cast<float>(fEntry8);
		float fSlow19 = 0.22f * (1.2f * static_cast<float>(fEntry9) + 1.5f) * fSlow18;
		float fSlow20 = 0.5f * static_cast<float>(fEntry10);
		float fSlow21 = tanhf(fSlow20);
		float fSlow22 = 0.34f * fSlow18;
		float fSlow23 = static_cast<float>(fEntry11);
		float fSlow24 = 1.9f * fSlow23 + 1.0f;
		float fSlow25 = std::pow(1e+01f, 0.2f * fSlow23);
		float fSlow26 = 1.0f - 0.22f * fSlow23;
		float fSlow27 = fConst1 * static_cast<float>(fHslider7);
		for (int i0 = 0; i0 < count; i0 = i0 + 1) {
			fRec0[0] = fSlow0 + fConst2 * fRec0[1];
			float fTemp0 = std::pow(1e+01f, fSlow2 * fRec0[0]);
			float fTemp1 = std::sqrt(fTemp0);
			fRec6[0] = fSlow3 + fConst2 * fRec6[1];
			float fTemp2 = std::pow(1e+01f, fSlow4 * fRec6[0]);
			float fTemp3 = std::sqrt(fTemp2);
			fRec16[0] = fSlow8 + fConst2 * fRec16[1];
			float fTemp4 = std::pow(1e+01f, 0.05f * fRec16[0]);
			float fTemp5 = std::sqrt(fTemp4);
			fRec22[0] = fSlow9 + fConst2 * fRec22[1];
			float fTemp6 = std::pow(1e+01f, 0.05f * (fRec22[0] + -2.5f));
			fRec27[0] = fSlow10 + fConst2 * fRec27[1];
			float fTemp7 = std::pow(1e+01f, 0.05f * fRec27[0]);
			float fTemp8 = std::sqrt(fTemp7);
			fRec33[0] = fSlow12 + fConst2 * fRec33[1];
			float fTemp9 = 72.0f * fRec33[0] + 8.0f;
			fRec44[0] = fSlow13 + fConst2 * fRec44[1];
			float fTemp10 = std::pow(1e+01f, 0.05f * fRec44[0]);
			float fTemp11 = static_cast<float>(input0[i0]);
			fRec45[0] = std::max<float>(0.995f * fRec45[1], std::fabs(fTemp11));
			fRec43[0] = fConst1 * static_cast<float>(fRec45[0] > fTemp10) + fConst2 * fRec43[1];
			float fTemp12 = fTemp11 * fRec43[0] * fRec33[0] - (fConst18 * fRec38[1] + fRec39[1]);
			fRec38[0] = fRec38[1] + fConst21 * fTemp12;
			float fTemp13 = fRec38[1] + fConst20 * fTemp12;
			fRec39[0] = fRec39[1] + fConst22 * fTemp13;
			float fTemp14 = fConst17 * fTemp13;
			float fRec40 = fRec39[1] + fTemp14;
			float fTemp15 = fConst23 * fTemp12;
			float fRec41 = fTemp15;
			float fRec42 = fTemp13;
			float fTemp16 = fRec41 + fSlow15 * fRec40 + fSlow16 * fRec42 - (fConst16 * fRec34[1] + fRec35[1]);
			fRec34[0] = fRec34[1] + fConst26 * fTemp16;
			float fTemp17 = fRec34[1] + fConst25 * fTemp16;
			fRec35[0] = fRec35[1] + fConst27 * fTemp17;
			float fRec36 = fTemp17;
			float fTemp18 = fConst28 * fTemp16;
			float fTemp19 = fConst15 * fTemp17;
			float fRec37 = fTemp19 + fRec35[1] + fTemp18;
			float fTemp20 = fSlow19 * (fRec37 + fSlow17 * fRec36) * fTemp9;
			float fTemp21 = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp20)))) * ((fTemp20 > 0.0f) ? 1.0f : ((fTemp20 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp20) - fSlow21 : tanhf(fTemp20)));
			float fTemp22 = fTemp21 - (fConst30 * fRec46[1] + fRec47[1]);
			fRec46[0] = fRec46[1] + fConst33 * fTemp22;
			float fTemp23 = fRec46[1] + fConst32 * fTemp22;
			fRec47[0] = fRec47[1] + fConst34 * fTemp23;
			float fTemp24 = fConst35 * fTemp22;
			float fRec48 = fTemp24;
			float fTemp25 = fSlow22 * fTemp9 * ((iSlow11) ? fRec48 : fTemp21) + 0.03f;
			float fTemp26 = 0.3128f * fTemp9 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp25)))) * ((fTemp25 > 0.0f) ? 1.0f : ((fTemp25 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp25) - fSlow21 : tanhf(fTemp25)));
			float fTemp27 = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp26)))) * ((fTemp26 > 0.0f) ? 1.0f : ((fTemp26 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp26) - fSlow21 : tanhf(fTemp26))) - (fConst14 * fRec28[1] + fRec29[1]);
			fRec28[0] = fRec28[1] + fConst38 * fTemp27;
			float fTemp28 = fRec28[1] + fConst37 * fTemp27;
			fRec29[0] = fRec29[1] + fConst39 * fTemp28;
			float fTemp29 = fConst13 * fTemp28;
			float fRec30 = fRec29[1] + fTemp29;
			float fTemp30 = fConst40 * fTemp27;
			float fRec31 = fTemp30;
			float fRec32 = fTemp28;
			float fTemp31 = fRec31 + fRec30 * fTemp7 + 1.4144272f * fRec32 * fTemp8 - (fConst12 * fRec23[1] + fRec24[1]);
			fRec23[0] = fRec23[1] + fConst43 * fTemp31;
			float fTemp32 = fRec23[1] + fConst42 * fTemp31;
			fRec24[0] = fRec24[1] + fConst44 * fTemp32;
			float fRec25 = fTemp32;
			float fTemp33 = fConst45 * fTemp31;
			float fTemp34 = fConst11 * fTemp32;
			float fRec26 = fTemp34 + fRec24[1] + fTemp33;
			float fTemp35 = fRec26 + fRec25 * fTemp6 - (fConst10 * fRec17[1] + fRec18[1]);
			fRec17[0] = fRec17[1] + fConst48 * fTemp35;
			float fTemp36 = fRec17[1] + fConst47 * fTemp35;
			fRec18[0] = fRec18[1] + fConst49 * fTemp36;
			float fTemp37 = fConst9 * fTemp36;
			float fRec19 = fRec18[1] + fTemp37;
			float fTemp38 = fConst50 * fTemp35;
			float fRec20 = fTemp38;
			float fRec21 = fTemp36;
			float fTemp39 = fSlow24 * (fRec19 + fRec20 * fTemp4 + 1.4144272f * fRec21 * fTemp5) - (fConst8 * fRec12[1] + fRec13[1]);
			fRec12[0] = fRec12[1] + fConst53 * fTemp39;
			float fTemp40 = fRec12[1] + fConst52 * fTemp39;
			fRec13[0] = fRec13[1] + fConst54 * fTemp40;
			float fRec14 = fTemp40;
			float fTemp41 = fConst55 * fTemp39;
			float fTemp42 = fConst7 * fTemp40;
			float fRec15 = fTemp42 + fRec13[1] + fTemp41;
			float fTemp43 = fRec15 + fSlow25 * fRec14;
			float fTemp44 = fTemp11 * fRec33[0] - (fConst18 * fRec71[1] + fRec72[1]);
			fRec71[0] = fRec71[1] + fConst21 * fTemp44;
			float fTemp45 = fRec71[1] + fConst20 * fTemp44;
			fRec72[0] = fRec72[1] + fConst22 * fTemp45;
			float fTemp46 = fConst17 * fTemp45;
			float fRec73 = fRec72[1] + fTemp46;
			float fTemp47 = fConst23 * fTemp44;
			float fRec74 = fTemp47;
			float fRec75 = fTemp45;
			float fTemp48 = fRec74 + fSlow15 * fRec73 + fSlow16 * fRec75 - (fConst16 * fRec67[1] + fRec68[1]);
			fRec67[0] = fRec67[1] + fConst26 * fTemp48;
			float fTemp49 = fRec67[1] + fConst25 * fTemp48;
			fRec68[0] = fRec68[1] + fConst27 * fTemp49;
			float fRec69 = fTemp49;
			float fTemp50 = fConst28 * fTemp48;
			float fTemp51 = fConst15 * fTemp49;
			float fRec70 = fTemp51 + fRec68[1] + fTemp50;
			float fTemp52 = fSlow19 * fTemp9 * (fRec70 + fSlow17 * fRec69);
			float fTemp53 = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp52)))) * ((fTemp52 > 0.0f) ? 1.0f : ((fTemp52 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp52) - fSlow21 : tanhf(fTemp52)));
			float fTemp54 = fTemp53 - (fConst30 * fRec76[1] + fRec77[1]);
			fRec76[0] = fRec76[1] + fConst33 * fTemp54;
			float fTemp55 = fRec76[1] + fConst32 * fTemp54;
			fRec77[0] = fRec77[1] + fConst34 * fTemp55;
			float fTemp56 = fConst35 * fTemp54;
			float fRec78 = fTemp56;
			float fTemp57 = fSlow22 * fTemp9 * ((iSlow11) ? fRec78 : fTemp53) + 0.03f;
			float fTemp58 = 0.3128f * fTemp9 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp57)))) * ((fTemp57 > 0.0f) ? 1.0f : ((fTemp57 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp57) - fSlow21 : tanhf(fTemp57)));
			float fTemp59 = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp58)))) * ((fTemp58 > 0.0f) ? 1.0f : ((fTemp58 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp58) - fSlow21 : tanhf(fTemp58))) - (fConst14 * fRec62[1] + fRec63[1]);
			fRec62[0] = fRec62[1] + fConst38 * fTemp59;
			float fTemp60 = fRec62[1] + fConst37 * fTemp59;
			fRec63[0] = fRec63[1] + fConst39 * fTemp60;
			float fTemp61 = fConst13 * fTemp60;
			float fRec64 = fRec63[1] + fTemp61;
			float fTemp62 = fConst40 * fTemp59;
			float fRec65 = fTemp62;
			float fRec66 = fTemp60;
			float fTemp63 = fRec65 + fRec64 * fTemp7 + 1.4144272f * fRec66 * fTemp8 - (fConst12 * fRec58[1] + fRec59[1]);
			fRec58[0] = fRec58[1] + fConst43 * fTemp63;
			float fTemp64 = fRec58[1] + fConst42 * fTemp63;
			fRec59[0] = fRec59[1] + fConst44 * fTemp64;
			float fRec60 = fTemp64;
			float fTemp65 = fConst45 * fTemp63;
			float fTemp66 = fConst11 * fTemp64;
			float fRec61 = fTemp66 + fRec59[1] + fTemp65;
			float fTemp67 = fRec61 + fRec60 * fTemp6 - (fConst10 * fRec53[1] + fRec54[1]);
			fRec53[0] = fRec53[1] + fConst48 * fTemp67;
			float fTemp68 = fRec53[1] + fConst47 * fTemp67;
			fRec54[0] = fRec54[1] + fConst49 * fTemp68;
			float fTemp69 = fConst9 * fTemp68;
			float fRec55 = fRec54[1] + fTemp69;
			float fTemp70 = fConst50 * fTemp67;
			float fRec56 = fTemp70;
			float fRec57 = fTemp68;
			float fTemp71 = fSlow24 * (fRec55 + fRec56 * fTemp4 + 1.4144272f * fRec57 * fTemp5) - (fConst8 * fRec49[1] + fRec50[1]);
			fRec49[0] = fRec49[1] + fConst53 * fTemp71;
			float fTemp72 = fRec49[1] + fConst52 * fTemp71;
			fRec50[0] = fRec50[1] + fConst54 * fTemp72;
			float fRec51 = fTemp72;
			float fTemp73 = fConst55 * fTemp71;
			float fTemp74 = fConst7 * fTemp72;
			float fRec52 = fTemp74 + fRec50[1] + fTemp73;
			float fTemp75 = fRec52 + fSlow25 * fRec51;
			float fTemp76 = ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp75)))) * ((fTemp75 > 0.0f) ? 1.0f : ((fTemp75 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp75) - fSlow21 : tanhf(fTemp75)));
			fRec80[0] = std::max<float>(0.995f * fRec80[1], std::fabs(fSlow26 * fTemp76));
			fRec79[0] = fConst1 * static_cast<float>(fRec80[0] > fTemp10) + fConst2 * fRec79[1];
			float fTemp77 = ((iSlow5) ? fSlow26 * fRec79[0] * fTemp76 : fSlow26 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp43)))) * ((fTemp43 > 0.0f) ? 1.0f : ((fTemp43 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp43) - fSlow21 : tanhf(fTemp43)))) - (fConst6 * fRec7[1] + fRec8[1]);
			fRec7[0] = fRec7[1] + fConst58 * fTemp77;
			float fTemp78 = fRec7[1] + fConst57 * fTemp77;
			fRec8[0] = fRec8[1] + fConst59 * fTemp78;
			float fTemp79 = fConst5 * fTemp78;
			float fRec9 = fRec8[1] + fTemp79;
			float fTemp80 = fConst60 * fTemp77;
			float fRec10 = fTemp80;
			float fRec11 = fTemp78;
			float fTemp81 = fRec10 + fRec9 * fTemp2 + 1.25f * fRec11 * fTemp3 - (fConst4 * fRec1[1] + fRec2[1]);
			fRec1[0] = fRec1[1] + fConst63 * fTemp81;
			float fTemp82 = fRec1[1] + fConst62 * fTemp81;
			fRec2[0] = fRec2[1] + fConst64 * fTemp82;
			float fTemp83 = fConst3 * fTemp82;
			float fRec3 = fRec2[1] + fTemp83;
			float fTemp84 = fConst65 * fTemp81;
			float fRec4 = fTemp84;
			float fRec5 = fTemp82;
			fRec81[0] = fSlow27 + fConst2 * fRec81[1];
			output0[i0] = static_cast<FAUSTFLOAT>(fRec81[0] * (fRec3 + fRec4 * fTemp0 + 1.4285715f * fRec5 * fTemp1));
			float fTemp85 = static_cast<float>(input1[i0]);
			fRec120[0] = std::max<float>(0.995f * fRec120[1], std::fabs(fTemp85));
			fRec119[0] = fConst1 * static_cast<float>(fRec120[0] > fTemp10) + fConst2 * fRec119[1];
			float fTemp86 = fTemp85 * fRec33[0];
			float fTemp87 = fTemp86 * fRec119[0] - (fConst18 * fRec114[1] + fRec115[1]);
			fRec114[0] = fRec114[1] + fConst21 * fTemp87;
			float fTemp88 = fRec114[1] + fConst20 * fTemp87;
			fRec115[0] = fRec115[1] + fConst22 * fTemp88;
			float fTemp89 = fConst17 * fTemp88;
			float fRec116 = fRec115[1] + fTemp89;
			float fTemp90 = fConst23 * fTemp87;
			float fRec117 = fTemp90;
			float fRec118 = fTemp88;
			float fTemp91 = fRec117 + fSlow15 * fRec116 + fSlow16 * fRec118 - (fConst16 * fRec110[1] + fRec111[1]);
			fRec110[0] = fRec110[1] + fConst26 * fTemp91;
			float fTemp92 = fRec110[1] + fConst25 * fTemp91;
			fRec111[0] = fRec111[1] + fConst27 * fTemp92;
			float fRec112 = fTemp92;
			float fTemp93 = fConst28 * fTemp91;
			float fTemp94 = fConst15 * fTemp92;
			float fRec113 = fTemp94 + fRec111[1] + fTemp93;
			float fTemp95 = fSlow19 * fTemp9 * (fRec113 + fSlow17 * fRec112);
			float fTemp96 = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp95)))) * ((fTemp95 > 0.0f) ? 1.0f : ((fTemp95 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp95) - fSlow21 : tanhf(fTemp95)));
			float fTemp97 = fTemp96 - (fConst30 * fRec121[1] + fRec122[1]);
			fRec121[0] = fRec121[1] + fConst33 * fTemp97;
			float fTemp98 = fRec121[1] + fConst32 * fTemp97;
			fRec122[0] = fRec122[1] + fConst34 * fTemp98;
			float fTemp99 = fConst35 * fTemp97;
			float fRec123 = fTemp99;
			float fTemp100 = fSlow22 * fTemp9 * ((iSlow11) ? fRec123 : fTemp96) + 0.03f;
			float fTemp101 = 0.3128f * fTemp9 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp100)))) * ((fTemp100 > 0.0f) ? 1.0f : ((fTemp100 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp100) - fSlow21 : tanhf(fTemp100)));
			float fTemp102 = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp101)))) * ((fTemp101 > 0.0f) ? 1.0f : ((fTemp101 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp101) - fSlow21 : tanhf(fTemp101))) - (fConst14 * fRec105[1] + fRec106[1]);
			fRec105[0] = fRec105[1] + fConst38 * fTemp102;
			float fTemp103 = fRec105[1] + fConst37 * fTemp102;
			fRec106[0] = fRec106[1] + fConst39 * fTemp103;
			float fTemp104 = fConst13 * fTemp103;
			float fRec107 = fRec106[1] + fTemp104;
			float fTemp105 = fConst40 * fTemp102;
			float fRec108 = fTemp105;
			float fRec109 = fTemp103;
			float fTemp106 = fRec108 + fRec107 * fTemp7 + 1.4144272f * fRec109 * fTemp8 - (fConst12 * fRec101[1] + fRec102[1]);
			fRec101[0] = fRec101[1] + fConst43 * fTemp106;
			float fTemp107 = fRec101[1] + fConst42 * fTemp106;
			fRec102[0] = fRec102[1] + fConst44 * fTemp107;
			float fRec103 = fTemp107;
			float fTemp108 = fConst45 * fTemp106;
			float fTemp109 = fConst11 * fTemp107;
			float fRec104 = fTemp109 + fRec102[1] + fTemp108;
			float fTemp110 = fRec104 + fRec103 * fTemp6 - (fConst10 * fRec96[1] + fRec97[1]);
			fRec96[0] = fRec96[1] + fConst48 * fTemp110;
			float fTemp111 = fRec96[1] + fConst47 * fTemp110;
			fRec97[0] = fRec97[1] + fConst49 * fTemp111;
			float fTemp112 = fConst9 * fTemp111;
			float fRec98 = fRec97[1] + fTemp112;
			float fTemp113 = fConst50 * fTemp110;
			float fRec99 = fTemp113;
			float fRec100 = fTemp111;
			float fTemp114 = fSlow24 * (fRec98 + fRec99 * fTemp4 + 1.4144272f * fRec100 * fTemp5) - (fConst8 * fRec92[1] + fRec93[1]);
			fRec92[0] = fRec92[1] + fConst53 * fTemp114;
			float fTemp115 = fRec92[1] + fConst52 * fTemp114;
			fRec93[0] = fRec93[1] + fConst54 * fTemp115;
			float fRec94 = fTemp115;
			float fTemp116 = fConst55 * fTemp114;
			float fTemp117 = fConst7 * fTemp115;
			float fRec95 = fTemp117 + fRec93[1] + fTemp116;
			float fTemp118 = fRec95 + fSlow25 * fRec94;
			float fTemp119 = fTemp86 - (fConst18 * fRec146[1] + fRec147[1]);
			fRec146[0] = fRec146[1] + fConst21 * fTemp119;
			float fTemp120 = fRec146[1] + fConst20 * fTemp119;
			fRec147[0] = fRec147[1] + fConst22 * fTemp120;
			float fTemp121 = fConst17 * fTemp120;
			float fRec148 = fRec147[1] + fTemp121;
			float fTemp122 = fConst23 * fTemp119;
			float fRec149 = fTemp122;
			float fRec150 = fTemp120;
			float fTemp123 = fRec149 + fSlow15 * fRec148 + fSlow16 * fRec150 - (fConst16 * fRec142[1] + fRec143[1]);
			fRec142[0] = fRec142[1] + fConst26 * fTemp123;
			float fTemp124 = fRec142[1] + fConst25 * fTemp123;
			fRec143[0] = fRec143[1] + fConst27 * fTemp124;
			float fRec144 = fTemp124;
			float fTemp125 = fConst28 * fTemp123;
			float fTemp126 = fConst15 * fTemp124;
			float fRec145 = fTemp126 + fRec143[1] + fTemp125;
			float fTemp127 = fSlow19 * fTemp9 * (fRec145 + fSlow17 * fRec144);
			float fTemp128 = 0.78f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp127)))) * ((fTemp127 > 0.0f) ? 1.0f : ((fTemp127 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp127) - fSlow21 : tanhf(fTemp127)));
			float fTemp129 = fTemp128 - (fConst30 * fRec151[1] + fRec152[1]);
			fRec151[0] = fRec151[1] + fConst33 * fTemp129;
			float fTemp130 = fRec151[1] + fConst32 * fTemp129;
			fRec152[0] = fRec152[1] + fConst34 * fTemp130;
			float fTemp131 = fConst35 * fTemp129;
			float fRec153 = fTemp131;
			float fTemp132 = fSlow22 * fTemp9 * ((iSlow11) ? fRec153 : fTemp128) + 0.03f;
			float fTemp133 = 0.3128f * fTemp9 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp132)))) * ((fTemp132 > 0.0f) ? 1.0f : ((fTemp132 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp132) - fSlow21 : tanhf(fTemp132)));
			float fTemp134 = 0.62f * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp133)))) * ((fTemp133 > 0.0f) ? 1.0f : ((fTemp133 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp133) - fSlow21 : tanhf(fTemp133))) - (fConst14 * fRec137[1] + fRec138[1]);
			fRec137[0] = fRec137[1] + fConst38 * fTemp134;
			float fTemp135 = fRec137[1] + fConst37 * fTemp134;
			fRec138[0] = fRec138[1] + fConst39 * fTemp135;
			float fTemp136 = fConst13 * fTemp135;
			float fRec139 = fRec138[1] + fTemp136;
			float fTemp137 = fConst40 * fTemp134;
			float fRec140 = fTemp137;
			float fRec141 = fTemp135;
			float fTemp138 = fRec140 + fRec139 * fTemp7 + 1.4144272f * fRec141 * fTemp8 - (fConst12 * fRec133[1] + fRec134[1]);
			fRec133[0] = fRec133[1] + fConst43 * fTemp138;
			float fTemp139 = fRec133[1] + fConst42 * fTemp138;
			fRec134[0] = fRec134[1] + fConst44 * fTemp139;
			float fRec135 = fTemp139;
			float fTemp140 = fConst45 * fTemp138;
			float fTemp141 = fConst11 * fTemp139;
			float fRec136 = fTemp141 + fRec134[1] + fTemp140;
			float fTemp142 = fRec136 + fRec135 * fTemp6 - (fConst10 * fRec128[1] + fRec129[1]);
			fRec128[0] = fRec128[1] + fConst48 * fTemp142;
			float fTemp143 = fRec128[1] + fConst47 * fTemp142;
			fRec129[0] = fRec129[1] + fConst49 * fTemp143;
			float fTemp144 = fConst9 * fTemp143;
			float fRec130 = fRec129[1] + fTemp144;
			float fTemp145 = fConst50 * fTemp142;
			float fRec131 = fTemp145;
			float fRec132 = fTemp143;
			float fTemp146 = fSlow24 * (fRec130 + fRec131 * fTemp4 + 1.4144272f * fRec132 * fTemp5) - (fConst8 * fRec124[1] + fRec125[1]);
			fRec124[0] = fRec124[1] + fConst53 * fTemp146;
			float fTemp147 = fRec124[1] + fConst52 * fTemp146;
			fRec125[0] = fRec125[1] + fConst54 * fTemp147;
			float fRec126 = fTemp147;
			float fTemp148 = fConst55 * fTemp146;
			float fTemp149 = fConst7 * fTemp147;
			float fRec127 = fTemp149 + fRec125[1] + fTemp148;
			float fTemp150 = fRec127 + fSlow25 * fRec126;
			float fTemp151 = ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp150)))) * ((fTemp150 > 0.0f) ? 1.0f : ((fTemp150 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp150) - fSlow21 : tanhf(fTemp150)));
			fRec155[0] = std::max<float>(0.995f * fRec155[1], std::fabs(fSlow26 * fTemp151));
			fRec154[0] = fConst1 * static_cast<float>(fRec155[0] > fTemp10) + fConst2 * fRec154[1];
			float fTemp152 = ((iSlow5) ? fSlow26 * fRec154[0] * fTemp151 : fSlow26 * ((iSlow6) ? (1.0f - std::exp(-(std::fabs(fTemp118)))) * ((fTemp118 > 0.0f) ? 1.0f : ((fTemp118 < 0.0f) ? -1.0f : 0.0f)) : ((iSlow7) ? tanhf(fSlow20 + fTemp118) - fSlow21 : tanhf(fTemp118)))) - (fConst6 * fRec87[1] + fRec88[1]);
			fRec87[0] = fRec87[1] + fConst58 * fTemp152;
			float fTemp153 = fRec87[1] + fConst57 * fTemp152;
			fRec88[0] = fRec88[1] + fConst59 * fTemp153;
			float fTemp154 = fConst5 * fTemp153;
			float fRec89 = fRec88[1] + fTemp154;
			float fTemp155 = fConst60 * fTemp152;
			float fRec90 = fTemp155;
			float fRec91 = fTemp153;
			float fTemp156 = fRec90 + fRec89 * fTemp2 + 1.25f * fRec91 * fTemp3 - (fConst4 * fRec82[1] + fRec83[1]);
			fRec82[0] = fRec82[1] + fConst63 * fTemp156;
			float fTemp157 = fRec82[1] + fConst62 * fTemp156;
			fRec83[0] = fRec83[1] + fConst64 * fTemp157;
			float fTemp158 = fConst3 * fTemp157;
			float fRec84 = fRec83[1] + fTemp158;
			float fTemp159 = fConst65 * fTemp156;
			float fRec85 = fTemp159;
			float fRec86 = fTemp157;
			output1[i0] = static_cast<FAUSTFLOAT>(fRec81[0] * (fRec84 + fRec85 * fTemp0 + 1.4285715f * fRec86 * fTemp1));
			fRec0[1] = fRec0[0];
			fRec6[1] = fRec6[0];
			fRec16[1] = fRec16[0];
			fRec22[1] = fRec22[0];
			fRec27[1] = fRec27[0];
			fRec33[1] = fRec33[0];
			fRec44[1] = fRec44[0];
			fRec45[1] = fRec45[0];
			fRec43[1] = fRec43[0];
			fRec38[1] = fRec38[0];
			fRec39[1] = fRec39[0];
			fRec34[1] = fRec34[0];
			fRec35[1] = fRec35[0];
			fRec46[1] = fRec46[0];
			fRec47[1] = fRec47[0];
			fRec28[1] = fRec28[0];
			fRec29[1] = fRec29[0];
			fRec23[1] = fRec23[0];
			fRec24[1] = fRec24[0];
			fRec17[1] = fRec17[0];
			fRec18[1] = fRec18[0];
			fRec12[1] = fRec12[0];
			fRec13[1] = fRec13[0];
			fRec71[1] = fRec71[0];
			fRec72[1] = fRec72[0];
			fRec67[1] = fRec67[0];
			fRec68[1] = fRec68[0];
			fRec76[1] = fRec76[0];
			fRec77[1] = fRec77[0];
			fRec62[1] = fRec62[0];
			fRec63[1] = fRec63[0];
			fRec58[1] = fRec58[0];
			fRec59[1] = fRec59[0];
			fRec53[1] = fRec53[0];
			fRec54[1] = fRec54[0];
			fRec49[1] = fRec49[0];
			fRec50[1] = fRec50[0];
			fRec80[1] = fRec80[0];
			fRec79[1] = fRec79[0];
			fRec7[1] = fRec7[0];
			fRec8[1] = fRec8[0];
			fRec1[1] = fRec1[0];
			fRec2[1] = fRec2[0];
			fRec81[1] = fRec81[0];
			fRec120[1] = fRec120[0];
			fRec119[1] = fRec119[0];
			fRec114[1] = fRec114[0];
			fRec115[1] = fRec115[0];
			fRec110[1] = fRec110[0];
			fRec111[1] = fRec111[0];
			fRec121[1] = fRec121[0];
			fRec122[1] = fRec122[0];
			fRec105[1] = fRec105[0];
			fRec106[1] = fRec106[0];
			fRec101[1] = fRec101[0];
			fRec102[1] = fRec102[0];
			fRec96[1] = fRec96[0];
			fRec97[1] = fRec97[0];
			fRec92[1] = fRec92[0];
			fRec93[1] = fRec93[0];
			fRec146[1] = fRec146[0];
			fRec147[1] = fRec147[0];
			fRec142[1] = fRec142[0];
			fRec143[1] = fRec143[0];
			fRec151[1] = fRec151[0];
			fRec152[1] = fRec152[0];
			fRec137[1] = fRec137[0];
			fRec138[1] = fRec138[0];
			fRec133[1] = fRec133[0];
			fRec134[1] = fRec134[0];
			fRec128[1] = fRec128[0];
			fRec129[1] = fRec129[0];
			fRec124[1] = fRec124[0];
			fRec125[1] = fRec125[0];
			fRec155[1] = fRec155[0];
			fRec154[1] = fRec154[0];
			fRec87[1] = fRec87[0];
			fRec88[1] = fRec88[0];
			fRec82[1] = fRec82[0];
			fRec83[1] = fRec83[0];
		}
	}

};

#endif
