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
static float mlczerov_faustpower3_f(float value) {
	return value * value * value;
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
	float fRec29_perm[4];
	float fRec31_perm[4];
	FAUSTFLOAT fHslider2;
	float fRec32_perm[4];
	float fRec30_perm[4];
	float fConst3;
	float fConst4;
	FAUSTFLOAT fEntry0;
	FAUSTFLOAT fEntry1;
	FAUSTFLOAT fEntry2;
	float fConst5;
	float fConst6;
	float fConst7;
	float fConst8;
	float fRec24_perm[4];
	float fConst9;
	float fRec25_perm[4];
	float fConst10;
	FAUSTFLOAT fHslider3;
	float fRec33_perm[4];
	float fConst11;
	float fConst12;
	float fConst13;
	float fConst14;
	float fConst15;
	float fRec20_perm[4];
	float fConst16;
	float fRec21_perm[4];
	float fConst17;
	FAUSTFLOAT fHslider4;
	float fRec34_perm[4];
	float fConst18;
	float fConst19;
	float fConst20;
	float fConst21;
	float fConst22;
	float fRec15_perm[4];
	float fConst23;
	float fRec16_perm[4];
	float fConst24;
	FAUSTFLOAT fHslider5;
	float fRec35_perm[4];
	float fConst25;
	float fConst26;
	FAUSTFLOAT fEntry3;
	float fConst27;
	float fConst28;
	float fConst29;
	float fRec11_perm[4];
	float fConst30;
	float fRec12_perm[4];
	float fConst31;
	float fRec51_perm[4];
	float fRec52_perm[4];
	float fRec47_perm[4];
	float fRec48_perm[4];
	float fRec42_perm[4];
	float fRec43_perm[4];
	float fRec38_perm[4];
	float fRec39_perm[4];
	float fRec37_perm[4];
	float fRec36_perm[4];
	float fConst32;
	float fConst33;
	FAUSTFLOAT fEntry4;
	float fConst34;
	float fConst35;
	float fConst36;
	float fRec6_perm[4];
	float fConst37;
	float fRec7_perm[4];
	float fConst38;
	FAUSTFLOAT fHslider6;
	float fRec56_perm[4];
	float fConst39;
	float fConst40;
	FAUSTFLOAT fEntry5;
	float fConst41;
	float fConst42;
	float fConst43;
	float fRec1_perm[4];
	float fConst44;
	float fRec2_perm[4];
	float fConst45;
	FAUSTFLOAT fHslider7;
	float fRec57_perm[4];
	float fRec87_perm[4];
	float fRec86_perm[4];
	float fRec81_perm[4];
	float fRec82_perm[4];
	float fRec77_perm[4];
	float fRec78_perm[4];
	float fRec72_perm[4];
	float fRec73_perm[4];
	float fRec68_perm[4];
	float fRec69_perm[4];
	float fRec103_perm[4];
	float fRec104_perm[4];
	float fRec99_perm[4];
	float fRec100_perm[4];
	float fRec94_perm[4];
	float fRec95_perm[4];
	float fRec90_perm[4];
	float fRec91_perm[4];
	float fRec89_perm[4];
	float fRec88_perm[4];
	float fRec63_perm[4];
	float fRec64_perm[4];
	float fRec58_perm[4];
	float fRec59_perm[4];
	
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
		fConst3 = std::tan(376.99112f / fConst0);
		fConst4 = fConst3 + 1.4144272f;
		fConst5 = tanhf(0.25f);
		fConst6 = fConst3 * fConst4 + 1.0f;
		fConst7 = fConst3 / fConst6;
		fConst8 = 2.0f * fConst7;
		fConst9 = 2.0f * fConst3;
		fConst10 = 1.0f / fConst6;
		fConst11 = std::tan(2387.6104f / fConst0);
		fConst12 = fConst11 + 1.1764706f;
		fConst13 = fConst11 * fConst12 + 1.0f;
		fConst14 = fConst11 / fConst13;
		fConst15 = 2.0f * fConst14;
		fConst16 = 2.0f * fConst11;
		fConst17 = 1.0f / fConst13;
		fConst18 = std::tan(10681.415f / fConst0);
		fConst19 = fConst18 + 1.4144272f;
		fConst20 = fConst18 * fConst19 + 1.0f;
		fConst21 = fConst18 / fConst20;
		fConst22 = 2.0f * fConst21;
		fConst23 = 2.0f * fConst18;
		fConst24 = 1.0f / fConst20;
		fConst25 = std::tan(2984.513f / fConst0);
		fConst26 = fConst25 + 0.8333333f;
		fConst27 = fConst25 * fConst26 + 1.0f;
		fConst28 = fConst25 / fConst27;
		fConst29 = 2.0f * fConst28;
		fConst30 = 2.0f * fConst25;
		fConst31 = 1.0f / fConst27;
		fConst32 = std::tan(298.4513f / fConst0);
		fConst33 = fConst32 + 1.25f;
		fConst34 = fConst32 * fConst33 + 1.0f;
		fConst35 = fConst32 / fConst34;
		fConst36 = 2.0f * fConst35;
		fConst37 = 2.0f * fConst32;
		fConst38 = 1.0f / fConst34;
		fConst39 = std::tan(11309.733f / fConst0);
		fConst40 = fConst39 + 1.4285715f;
		fConst41 = fConst39 * fConst40 + 1.0f;
		fConst42 = fConst39 / fConst41;
		fConst43 = 2.0f * fConst42;
		fConst44 = 2.0f * fConst39;
		fConst45 = 1.0f / fConst41;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(0.5011872f);
		fHslider1 = static_cast<FAUSTFLOAT>(0.25118864f);
		fHslider2 = static_cast<FAUSTFLOAT>(-8e+01f);
		fEntry0 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry1 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry2 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider5 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry3 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry4 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider6 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry5 = static_cast<FAUSTFLOAT>(1.0f);
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
			fRec31_perm[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 4; l3 = l3 + 1) {
			fRec32_perm[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 4; l4 = l4 + 1) {
			fRec30_perm[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 4; l5 = l5 + 1) {
			fRec24_perm[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 4; l6 = l6 + 1) {
			fRec25_perm[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 4; l7 = l7 + 1) {
			fRec33_perm[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 4; l8 = l8 + 1) {
			fRec20_perm[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 4; l9 = l9 + 1) {
			fRec21_perm[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 4; l10 = l10 + 1) {
			fRec34_perm[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 4; l11 = l11 + 1) {
			fRec15_perm[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 4; l12 = l12 + 1) {
			fRec16_perm[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 4; l13 = l13 + 1) {
			fRec35_perm[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 4; l14 = l14 + 1) {
			fRec11_perm[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 4; l15 = l15 + 1) {
			fRec12_perm[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 4; l16 = l16 + 1) {
			fRec51_perm[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 4; l17 = l17 + 1) {
			fRec52_perm[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 4; l18 = l18 + 1) {
			fRec47_perm[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 4; l19 = l19 + 1) {
			fRec48_perm[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 4; l20 = l20 + 1) {
			fRec42_perm[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 4; l21 = l21 + 1) {
			fRec43_perm[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 4; l22 = l22 + 1) {
			fRec38_perm[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 4; l23 = l23 + 1) {
			fRec39_perm[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 4; l24 = l24 + 1) {
			fRec37_perm[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 4; l25 = l25 + 1) {
			fRec36_perm[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 4; l26 = l26 + 1) {
			fRec6_perm[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 4; l27 = l27 + 1) {
			fRec7_perm[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 4; l28 = l28 + 1) {
			fRec56_perm[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 4; l29 = l29 + 1) {
			fRec1_perm[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 4; l30 = l30 + 1) {
			fRec2_perm[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 4; l31 = l31 + 1) {
			fRec57_perm[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 4; l32 = l32 + 1) {
			fRec87_perm[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 4; l33 = l33 + 1) {
			fRec86_perm[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 4; l34 = l34 + 1) {
			fRec81_perm[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 4; l35 = l35 + 1) {
			fRec82_perm[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 4; l36 = l36 + 1) {
			fRec77_perm[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 4; l37 = l37 + 1) {
			fRec78_perm[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 4; l38 = l38 + 1) {
			fRec72_perm[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 4; l39 = l39 + 1) {
			fRec73_perm[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 4; l40 = l40 + 1) {
			fRec68_perm[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 4; l41 = l41 + 1) {
			fRec69_perm[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 4; l42 = l42 + 1) {
			fRec103_perm[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 4; l43 = l43 + 1) {
			fRec104_perm[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 4; l44 = l44 + 1) {
			fRec99_perm[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 4; l45 = l45 + 1) {
			fRec100_perm[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 4; l46 = l46 + 1) {
			fRec94_perm[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 4; l47 = l47 + 1) {
			fRec95_perm[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 4; l48 = l48 + 1) {
			fRec90_perm[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 4; l49 = l49 + 1) {
			fRec91_perm[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 4; l50 = l50 + 1) {
			fRec89_perm[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 4; l51 = l51 + 1) {
			fRec88_perm[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 4; l52 = l52 + 1) {
			fRec63_perm[l52] = 0.0f;
		}
		for (int l53 = 0; l53 < 4; l53 = l53 + 1) {
			fRec64_perm[l53] = 0.0f;
		}
		for (int l54 = 0; l54 < 4; l54 = l54 + 1) {
			fRec58_perm[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 4; l55 = l55 + 1) {
			fRec59_perm[l55] = 0.0f;
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
		ui_interface->declare(&fHslider3, "unit", "dB");
		ui_interface->addHorizontalSlider("Bass", &fHslider3, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Bright", &fEntry2, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addNumEntry("Clip Type", &fEntry0, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider6, "unit", "dB");
		ui_interface->addHorizontalSlider("Depth", &fHslider6, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Feedback", &fEntry5, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Gain", &fHslider1, FAUSTFLOAT(0.25118864f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->addNumEntry("Gate Pos", &fEntry4, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider2, "unit", "dB");
		ui_interface->addHorizontalSlider("Gate", &fHslider2, FAUSTFLOAT(-8e+01f), FAUSTFLOAT(-8e+01f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("M45", &fEntry1, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Master", &fHslider0, FAUSTFLOAT(0.5011872f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->declare(&fHslider4, "unit", "dB");
		ui_interface->addHorizontalSlider("Middle", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider7, "unit", "dB");
		ui_interface->addHorizontalSlider("Presence", &fHslider7, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider5, "unit", "dB");
		ui_interface->addHorizontalSlider("Treble", &fHslider5, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("WARCLAW", &fEntry3, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
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
		float fRec31_tmp[36];
		float* fRec31 = &fRec31_tmp[4];
		float fSlow2 = fConst1 * static_cast<float>(fHslider2);
		float fRec32_tmp[36];
		float* fRec32 = &fRec32_tmp[4];
		float fZec0[32];
		float fRec30_tmp[36];
		float* fRec30 = &fRec30_tmp[4];
		int iSlow3 = static_cast<int>(static_cast<float>(fEntry0));
		int iSlow4 = iSlow3 >= 6;
		int iSlow5 = iSlow3 >= 3;
		int iSlow6 = iSlow3 >= 2;
		int iSlow7 = iSlow3 >= 1;
		float fZec1[32];
		float fZec2[32];
		float fSlow8 = 1.0f - 0.35f * static_cast<float>(fEntry1);
		float fSlow9 = 1.2f * static_cast<float>(fEntry2) + 1.5f;
		float fSlow10 = fSlow9 * fSlow8;
		float fSlow11 = 0.22f * fSlow10;
		float fZec3[32];
		int iSlow12 = iSlow3 >= 5;
		int iSlow13 = iSlow3 >= 4;
		float fSlow14 = 0.34557518f * fSlow10;
		float fZec4[32];
		float fZec5[32];
		float fZec6[32];
		float fZec7[32];
		float fSlow15 = mlczerov_faustpower2_f(fSlow9) * mlczerov_faustpower2_f(fSlow8);
		float fSlow16 = 0.0484f * fSlow15;
		float fZec8[32];
		int iSlow17 = iSlow3 >= 9;
		int iSlow18 = iSlow3 >= 8;
		int iSlow19 = iSlow3 >= 7;
		float fZec9[32];
		float fSlow20 = 0.0035493334f * fSlow15;
		int iSlow21 = iSlow3 >= 10;
		float fSlow22 = 0.2652f * fSlow8;
		float fZec10[32];
		float fZec11[32];
		float fZec12[32];
		float fZec13[32];
		float fZec14[32];
		float fZec15[32];
		float fZec16[32];
		float fZec17[32];
		float fZec18[32];
		float fZec19[32];
		float fZec20[32];
		float fRec24_tmp[36];
		float* fRec24 = &fRec24_tmp[4];
		float fZec21[32];
		float fRec25_tmp[36];
		float* fRec25 = &fRec25_tmp[4];
		float fZec22[32];
		float fRec26[32];
		float fZec23[32];
		float fRec27[32];
		float fRec28[32];
		float fSlow23 = fConst1 * static_cast<float>(fHslider3);
		float fRec33_tmp[36];
		float* fRec33 = &fRec33_tmp[4];
		float fZec24[32];
		float fZec25[32];
		float fZec26[32];
		float fRec20_tmp[36];
		float* fRec20 = &fRec20_tmp[4];
		float fZec27[32];
		float fRec21_tmp[36];
		float* fRec21 = &fRec21_tmp[4];
		float fRec22[32];
		float fZec28[32];
		float fZec29[32];
		float fRec23[32];
		float fSlow24 = fConst1 * static_cast<float>(fHslider4);
		float fRec34_tmp[36];
		float* fRec34 = &fRec34_tmp[4];
		float fZec30[32];
		float fZec31[32];
		float fRec15_tmp[36];
		float* fRec15 = &fRec15_tmp[4];
		float fZec32[32];
		float fRec16_tmp[36];
		float* fRec16 = &fRec16_tmp[4];
		float fZec33[32];
		float fRec17[32];
		float fZec34[32];
		float fRec18[32];
		float fRec19[32];
		float fSlow25 = fConst1 * static_cast<float>(fHslider5);
		float fRec35_tmp[36];
		float* fRec35 = &fRec35_tmp[4];
		float fZec35[32];
		float fZec36[32];
		float fSlow26 = static_cast<float>(fEntry3);
		float fSlow27 = 1.9f * fSlow26 + 1.0f;
		float fZec37[32];
		float fRec11_tmp[36];
		float* fRec11 = &fRec11_tmp[4];
		float fZec38[32];
		float fRec12_tmp[36];
		float* fRec12 = &fRec12_tmp[4];
		float fRec13[32];
		float fZec39[32];
		float fZec40[32];
		float fRec14[32];
		float fZec41[32];
		float fZec42[32];
		float fZec43[32];
		float fZec44[32];
		float fZec45[32];
		float fZec46[32];
		float fZec47[32];
		float fZec48[32];
		float fZec49[32];
		float fZec50[32];
		float fZec51[32];
		float fZec52[32];
		float fZec53[32];
		float fZec54[32];
		float fZec55[32];
		float fZec56[32];
		float fRec51_tmp[36];
		float* fRec51 = &fRec51_tmp[4];
		float fZec57[32];
		float fRec52_tmp[36];
		float* fRec52 = &fRec52_tmp[4];
		float fZec58[32];
		float fRec53[32];
		float fZec59[32];
		float fRec54[32];
		float fRec55[32];
		float fZec60[32];
		float fRec47_tmp[36];
		float* fRec47 = &fRec47_tmp[4];
		float fZec61[32];
		float fRec48_tmp[36];
		float* fRec48 = &fRec48_tmp[4];
		float fRec49[32];
		float fZec62[32];
		float fZec63[32];
		float fRec50[32];
		float fZec64[32];
		float fRec42_tmp[36];
		float* fRec42 = &fRec42_tmp[4];
		float fZec65[32];
		float fRec43_tmp[36];
		float* fRec43 = &fRec43_tmp[4];
		float fZec66[32];
		float fRec44[32];
		float fZec67[32];
		float fRec45[32];
		float fRec46[32];
		float fZec68[32];
		float fRec38_tmp[36];
		float* fRec38 = &fRec38_tmp[4];
		float fZec69[32];
		float fRec39_tmp[36];
		float* fRec39 = &fRec39_tmp[4];
		float fRec40[32];
		float fZec70[32];
		float fZec71[32];
		float fRec41[32];
		float fSlow28 = std::pow(1e+01f, 0.2f * fSlow26);
		float fZec72[32];
		float fZec73[32];
		float fZec74[32];
		float fZec75[32];
		float fSlow29 = 1.0f - 0.22f * fSlow26;
		float fRec37_tmp[36];
		float* fRec37 = &fRec37_tmp[4];
		float fRec36_tmp[36];
		float* fRec36 = &fRec36_tmp[4];
		int iSlow30 = static_cast<int>(static_cast<float>(fEntry4));
		float fZec76[32];
		float fZec77[32];
		float fZec78[32];
		float fZec79[32];
		float fRec6_tmp[36];
		float* fRec6 = &fRec6_tmp[4];
		float fZec80[32];
		float fRec7_tmp[36];
		float* fRec7 = &fRec7_tmp[4];
		float fZec81[32];
		float fRec8[32];
		float fZec82[32];
		float fRec9[32];
		float fRec10[32];
		float fSlow31 = fConst1 * static_cast<float>(fHslider6);
		float fRec56_tmp[36];
		float* fRec56 = &fRec56_tmp[4];
		float fSlow32 = static_cast<float>(fEntry5);
		float fSlow33 = 0.05f * (1.25f - 0.35f * fSlow32);
		float fZec83[32];
		float fZec84[32];
		float fZec85[32];
		float fRec1_tmp[36];
		float* fRec1 = &fRec1_tmp[4];
		float fZec86[32];
		float fRec2_tmp[36];
		float* fRec2 = &fRec2_tmp[4];
		float fZec87[32];
		float fRec3[32];
		float fZec88[32];
		float fRec4[32];
		float fRec5[32];
		float fSlow34 = fConst1 * static_cast<float>(fHslider7);
		float fRec57_tmp[36];
		float* fRec57 = &fRec57_tmp[4];
		float fSlow35 = 0.05f * (0.25f * fSlow32 + 0.75f);
		float fZec89[32];
		float fZec90[32];
		float fRec87_tmp[36];
		float* fRec87 = &fRec87_tmp[4];
		float fRec86_tmp[36];
		float* fRec86 = &fRec86_tmp[4];
		float fZec91[32];
		float fZec92[32];
		float fZec93[32];
		float fZec94[32];
		float fZec95[32];
		float fZec96[32];
		float fZec97[32];
		float fZec98[32];
		float fZec99[32];
		float fZec100[32];
		float fZec101[32];
		float fZec102[32];
		float fZec103[32];
		float fZec104[32];
		float fZec105[32];
		float fZec106[32];
		float fZec107[32];
		float fZec108[32];
		float fRec81_tmp[36];
		float* fRec81 = &fRec81_tmp[4];
		float fZec109[32];
		float fRec82_tmp[36];
		float* fRec82 = &fRec82_tmp[4];
		float fZec110[32];
		float fRec83[32];
		float fZec111[32];
		float fRec84[32];
		float fRec85[32];
		float fZec112[32];
		float fRec77_tmp[36];
		float* fRec77 = &fRec77_tmp[4];
		float fZec113[32];
		float fRec78_tmp[36];
		float* fRec78 = &fRec78_tmp[4];
		float fRec79[32];
		float fZec114[32];
		float fZec115[32];
		float fRec80[32];
		float fZec116[32];
		float fRec72_tmp[36];
		float* fRec72 = &fRec72_tmp[4];
		float fZec117[32];
		float fRec73_tmp[36];
		float* fRec73 = &fRec73_tmp[4];
		float fZec118[32];
		float fRec74[32];
		float fZec119[32];
		float fRec75[32];
		float fRec76[32];
		float fZec120[32];
		float fRec68_tmp[36];
		float* fRec68 = &fRec68_tmp[4];
		float fZec121[32];
		float fRec69_tmp[36];
		float* fRec69 = &fRec69_tmp[4];
		float fRec70[32];
		float fZec122[32];
		float fZec123[32];
		float fRec71[32];
		float fZec124[32];
		float fZec125[32];
		float fZec126[32];
		float fZec127[32];
		float fZec128[32];
		float fZec129[32];
		float fZec130[32];
		float fZec131[32];
		float fZec132[32];
		float fZec133[32];
		float fZec134[32];
		float fZec135[32];
		float fZec136[32];
		float fZec137[32];
		float fZec138[32];
		float fZec139[32];
		float fRec103_tmp[36];
		float* fRec103 = &fRec103_tmp[4];
		float fZec140[32];
		float fRec104_tmp[36];
		float* fRec104 = &fRec104_tmp[4];
		float fZec141[32];
		float fRec105[32];
		float fZec142[32];
		float fRec106[32];
		float fRec107[32];
		float fZec143[32];
		float fRec99_tmp[36];
		float* fRec99 = &fRec99_tmp[4];
		float fZec144[32];
		float fRec100_tmp[36];
		float* fRec100 = &fRec100_tmp[4];
		float fRec101[32];
		float fZec145[32];
		float fZec146[32];
		float fRec102[32];
		float fZec147[32];
		float fRec94_tmp[36];
		float* fRec94 = &fRec94_tmp[4];
		float fZec148[32];
		float fRec95_tmp[36];
		float* fRec95 = &fRec95_tmp[4];
		float fZec149[32];
		float fRec96[32];
		float fZec150[32];
		float fRec97[32];
		float fRec98[32];
		float fZec151[32];
		float fRec90_tmp[36];
		float* fRec90 = &fRec90_tmp[4];
		float fZec152[32];
		float fRec91_tmp[36];
		float* fRec91 = &fRec91_tmp[4];
		float fRec92[32];
		float fZec153[32];
		float fZec154[32];
		float fRec93[32];
		float fZec155[32];
		float fZec156[32];
		float fZec157[32];
		float fZec158[32];
		float fRec89_tmp[36];
		float* fRec89 = &fRec89_tmp[4];
		float fRec88_tmp[36];
		float* fRec88 = &fRec88_tmp[4];
		float fZec159[32];
		float fZec160[32];
		float fZec161[32];
		float fZec162[32];
		float fRec63_tmp[36];
		float* fRec63 = &fRec63_tmp[4];
		float fZec163[32];
		float fRec64_tmp[36];
		float* fRec64 = &fRec64_tmp[4];
		float fZec164[32];
		float fRec65[32];
		float fZec165[32];
		float fRec66[32];
		float fRec67[32];
		float fZec166[32];
		float fRec58_tmp[36];
		float* fRec58 = &fRec58_tmp[4];
		float fZec167[32];
		float fRec59_tmp[36];
		float* fRec59 = &fRec59_tmp[4];
		float fZec168[32];
		float fRec60[32];
		float fZec169[32];
		float fRec61[32];
		float fRec62[32];
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
				fZec1[i] = 72.0f * fRec29[i] + 8.0f;
			}
			/* Vectorizable loop 2 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec91[i] = static_cast<float>(input1[i]) * fRec29[i];
			}
			/* Recursive loop 3 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec32_tmp[j6] = fRec32_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec32[i] = fSlow2 + fConst2 * fRec32[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec32_perm[j7] = fRec32_tmp[vsize + j7];
			}
			/* Vectorizable loop 4 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec5[i] = mlczerov_faustpower2_f(fRec29[i]);
			}
			/* Vectorizable loop 5 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec41[i] = static_cast<float>(input0[i]) * fRec29[i] * fZec1[i];
			}
			/* Vectorizable loop 6 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec124[i] = fZec91[i] * fZec1[i];
			}
			/* Recursive loop 7 */
			/* Pre code */
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec31_tmp[j4] = fRec31_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec31[i] = std::max<float>(0.995f * fRec31[i - 1], std::fabs(static_cast<float>(input0[i])));
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec31_perm[j5] = fRec31_tmp[vsize + j5];
			}
			/* Vectorizable loop 8 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::pow(1e+01f, 0.05f * fRec32[i]);
			}
			/* Vectorizable loop 9 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec4[i] = mlczerov_faustpower2_f(fZec1[i]);
			}
			/* Vectorizable loop 10 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec6[i] = mlczerov_faustpower2_f(static_cast<float>(input0[i]));
			}
			/* Vectorizable loop 11 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec42[i] = fSlow11 * fZec41[i];
			}
			/* Recursive loop 12 */
			/* Pre code */
			for (int j64 = 0; j64 < 4; j64 = j64 + 1) {
				fRec87_tmp[j64] = fRec87_perm[j64];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec87[i] = std::max<float>(0.995f * fRec87[i - 1], std::fabs(static_cast<float>(input1[i])));
			}
			/* Post code */
			for (int j65 = 0; j65 < 4; j65 = j65 + 1) {
				fRec87_perm[j65] = fRec87_tmp[vsize + j65];
			}
			/* Vectorizable loop 13 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec94[i] = mlczerov_faustpower2_f(static_cast<float>(input1[i])) * fZec5[i];
			}
			/* Vectorizable loop 14 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec125[i] = fSlow11 * fZec124[i];
			}
			/* Recursive loop 15 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec30_tmp[j8] = fRec30_perm[j8];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec30[i] = fConst1 * static_cast<float>(fRec31[i] > fZec0[i]) + fConst2 * fRec30[i - 1];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec30_perm[j9] = fRec30_tmp[vsize + j9];
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec43[i] = fZec6[i] * fZec5[i] * fZec4[i];
			}
			/* Vectorizable loop 17 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec44[i] = std::fabs(fZec42[i]);
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec45[i] = ((fZec42[i] > 0.0f) ? 1.0f : ((fZec42[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 19 */
			/* Pre code */
			for (int j66 = 0; j66 < 4; j66 = j66 + 1) {
				fRec86_tmp[j66] = fRec86_perm[j66];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec86[i] = fConst1 * static_cast<float>(fRec87[i] > fZec0[i]) + fConst2 * fRec86[i - 1];
			}
			/* Post code */
			for (int j67 = 0; j67 < 4; j67 = j67 + 1) {
				fRec86_perm[j67] = fRec86_tmp[vsize + j67];
			}
			/* Vectorizable loop 20 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec126[i] = fZec94[i] * fZec4[i];
			}
			/* Vectorizable loop 21 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec127[i] = std::fabs(fZec125[i]);
			}
			/* Vectorizable loop 22 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec128[i] = ((fZec125[i] > 0.0f) ? 1.0f : ((fZec125[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec2[i] = static_cast<float>(input0[i]) * fRec30[i] * fRec29[i] * fZec1[i];
			}
			/* Vectorizable loop 24 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec46[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec42[i], -0.6f), 0.35f) : fZec42[i] - 2.0f * fZec45[i] * std::max<float>(0.0f, fZec44[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec42[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec41[i] * (0.22f - fSlow20 * fZec43[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec44[i]))) * fZec45[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec41[i] / (fZec44[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec41[i] / std::sqrt(fSlow16 * fZec43[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec41[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec42[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec42[i], -0.5f), 0.5f) : tanhf(fZec42[i])))));
			}
			/* Vectorizable loop 25 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec92[i] = fZec91[i] * fRec86[i] * fZec1[i];
			}
			/* Vectorizable loop 26 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec129[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec125[i], -0.6f), 0.35f) : fZec125[i] - 2.0f * fZec128[i] * std::max<float>(0.0f, fZec127[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec125[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec124[i] * (0.22f - fSlow20 * fZec126[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec127[i]))) * fZec128[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec124[i] / (fZec127[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec124[i] / std::sqrt(fSlow16 * fZec126[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec124[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec125[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec125[i], -0.5f), 0.5f) : tanhf(fZec125[i])))));
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec3[i] = fSlow11 * fZec2[i];
			}
			/* Vectorizable loop 28 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec47[i] = fZec46[i] + 0.03f;
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec93[i] = fSlow11 * fZec92[i];
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec130[i] = fZec129[i] + 0.03f;
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec7[i] = fZec6[i] * mlczerov_faustpower2_f(fRec30[i]) * fZec5[i] * fZec4[i];
			}
			/* Vectorizable loop 32 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec8[i] = std::fabs(fZec3[i]);
			}
			/* Vectorizable loop 33 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = ((fZec3[i] > 0.0f) ? 1.0f : ((fZec3[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 34 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec48[i] = std::fabs(fZec47[i]);
			}
			/* Vectorizable loop 35 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec49[i] = ((fZec47[i] > 0.0f) ? 1.0f : ((fZec47[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 36 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec95[i] = fZec94[i] * mlczerov_faustpower2_f(fRec86[i]) * fZec4[i];
			}
			/* Vectorizable loop 37 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec96[i] = std::fabs(fZec93[i]);
			}
			/* Vectorizable loop 38 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec97[i] = ((fZec93[i] > 0.0f) ? 1.0f : ((fZec93[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 39 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec131[i] = std::fabs(fZec130[i]);
			}
			/* Vectorizable loop 40 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec132[i] = ((fZec130[i] > 0.0f) ? 1.0f : ((fZec130[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 41 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec3[i], -0.6f), 0.35f) : fZec3[i] - 2.0f * fZec9[i] * std::max<float>(0.0f, fZec8[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec3[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec2[i] * (0.22f - fSlow20 * fZec7[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec8[i]))) * fZec9[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec2[i] / (fZec8[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec2[i] / std::sqrt(fSlow16 * fZec7[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec2[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec3[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec3[i], -0.5f), 0.5f) : tanhf(fZec3[i])))));
			}
			/* Vectorizable loop 42 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec50[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec47[i], -0.6f), 0.35f) : fZec46[i] + (0.03f - 2.0f * fZec49[i] * std::max<float>(0.0f, fZec48[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec46[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec46[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec47[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec48[i]))) * fZec49[i]))) : ((iSlow5) ? ((iSlow12) ? fZec47[i] / (fZec48[i] + 1.0f) : ((iSlow13) ? fZec47[i] / std::sqrt(mlczerov_faustpower2_f(fZec47[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec47[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec47[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec47[i], -0.5f), 0.5f) : tanhf(fZec47[i])))));
			}
			/* Vectorizable loop 43 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec98[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec93[i], -0.6f), 0.35f) : fZec93[i] - 2.0f * fZec97[i] * std::max<float>(0.0f, fZec96[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec93[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec92[i] * (0.22f - fSlow20 * fZec95[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec96[i]))) * fZec97[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec92[i] / (fZec96[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec92[i] / std::sqrt(fSlow16 * fZec95[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec92[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec93[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec93[i], -0.5f), 0.5f) : tanhf(fZec93[i])))));
			}
			/* Vectorizable loop 44 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec133[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec130[i], -0.6f), 0.35f) : fZec129[i] + (0.03f - 2.0f * fZec132[i] * std::max<float>(0.0f, fZec131[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec129[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec129[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec130[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec131[i]))) * fZec132[i]))) : ((iSlow5) ? ((iSlow12) ? fZec130[i] / (fZec131[i] + 1.0f) : ((iSlow13) ? fZec130[i] / std::sqrt(mlczerov_faustpower2_f(fZec130[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec130[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec130[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec130[i], -0.5f), 0.5f) : tanhf(fZec130[i])))));
			}
			/* Vectorizable loop 45 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = fZec10[i] + 0.03f;
			}
			/* Vectorizable loop 46 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec51[i] = fZec1[i] * fZec50[i];
			}
			/* Vectorizable loop 47 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec99[i] = fZec98[i] + 0.03f;
			}
			/* Vectorizable loop 48 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec134[i] = fZec1[i] * fZec133[i];
			}
			/* Vectorizable loop 49 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = std::fabs(fZec11[i]);
			}
			/* Vectorizable loop 50 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec13[i] = ((fZec11[i] > 0.0f) ? 1.0f : ((fZec11[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 51 */
			/* Pre code */
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec33_tmp[j14] = fRec33_perm[j14];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec33[i] = fSlow23 + fConst2 * fRec33[i - 1];
			}
			/* Post code */
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec33_perm[j15] = fRec33_tmp[vsize + j15];
			}
			/* Vectorizable loop 52 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec52[i] = 0.3128f * fZec51[i];
			}
			/* Vectorizable loop 53 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec100[i] = std::fabs(fZec99[i]);
			}
			/* Vectorizable loop 54 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec101[i] = ((fZec99[i] > 0.0f) ? 1.0f : ((fZec99[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 55 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec135[i] = 0.3128f * fZec134[i];
			}
			/* Vectorizable loop 56 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec14[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec11[i], -0.6f), 0.35f) : fZec10[i] + (0.03f - 2.0f * fZec13[i] * std::max<float>(0.0f, fZec12[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec10[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec10[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec11[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec12[i]))) * fZec13[i]))) : ((iSlow5) ? ((iSlow12) ? fZec11[i] / (fZec12[i] + 1.0f) : ((iSlow13) ? fZec11[i] / std::sqrt(mlczerov_faustpower2_f(fZec11[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec11[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec11[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec11[i], -0.5f), 0.5f) : tanhf(fZec11[i])))));
			}
			/* Vectorizable loop 57 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec24[i] = std::pow(1e+01f, 0.05f * fRec33[i]);
			}
			/* Vectorizable loop 58 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec53[i] = fZec4[i] * mlczerov_faustpower2_f(fZec50[i]);
			}
			/* Vectorizable loop 59 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec54[i] = std::fabs(fZec52[i]);
			}
			/* Vectorizable loop 60 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec55[i] = ((fZec52[i] > 0.0f) ? 1.0f : ((fZec52[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 61 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec102[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec99[i], -0.6f), 0.35f) : fZec98[i] + (0.03f - 2.0f * fZec101[i] * std::max<float>(0.0f, fZec100[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec98[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec98[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec99[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec100[i]))) * fZec101[i]))) : ((iSlow5) ? ((iSlow12) ? fZec99[i] / (fZec100[i] + 1.0f) : ((iSlow13) ? fZec99[i] / std::sqrt(mlczerov_faustpower2_f(fZec99[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec99[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec99[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec99[i], -0.5f), 0.5f) : tanhf(fZec99[i])))));
			}
			/* Vectorizable loop 62 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec136[i] = fZec4[i] * mlczerov_faustpower2_f(fZec133[i]);
			}
			/* Vectorizable loop 63 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec137[i] = std::fabs(fZec135[i]);
			}
			/* Vectorizable loop 64 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec138[i] = ((fZec135[i] > 0.0f) ? 1.0f : ((fZec135[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 65 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec15[i] = fZec1[i] * fZec14[i];
			}
			/* Vectorizable loop 66 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec25[i] = std::sqrt(fZec24[i]);
			}
			/* Recursive loop 67 */
			/* Pre code */
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec34_tmp[j20] = fRec34_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec34[i] = fSlow24 + fConst2 * fRec34[i - 1];
			}
			/* Post code */
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec34_perm[j21] = fRec34_tmp[vsize + j21];
			}
			/* Recursive loop 68 */
			/* Pre code */
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec35_tmp[j26] = fRec35_perm[j26];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec35[i] = fSlow25 + fConst2 * fRec35[i - 1];
			}
			/* Post code */
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec35_perm[j27] = fRec35_tmp[vsize + j27];
			}
			/* Recursive loop 69 */
			/* Pre code */
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec51_tmp[j32] = fRec51_perm[j32];
			}
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec52_tmp[j34] = fRec52_perm[j34];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec56[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec52[i], -0.6f), 0.35f) : fZec52[i] - 2.0f * fZec55[i] * std::max<float>(0.0f, fZec54[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec52[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec51[i] * (0.3128f - 0.010201851f * fZec53[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec54[i]))) * fZec55[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec51[i] / (fZec54[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec51[i] / std::sqrt(0.09784384f * fZec53[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec51[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec52[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec52[i], -0.5f), 0.5f) : tanhf(fZec52[i]))))) - (fConst4 * fRec51[i - 1] + fRec52[i - 1]);
				fRec51[i] = fRec51[i - 1] + fConst8 * fZec56[i];
				fZec57[i] = fRec51[i - 1] + fConst7 * fZec56[i];
				fRec52[i] = fRec52[i - 1] + fConst9 * fZec57[i];
				fZec58[i] = fConst3 * fZec57[i];
				fRec53[i] = fRec52[i - 1] + fZec58[i];
				fZec59[i] = fConst10 * fZec56[i];
				fRec54[i] = fZec59[i];
				fRec55[i] = fZec57[i];
			}
			/* Post code */
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec51_perm[j33] = fRec51_tmp[vsize + j33];
			}
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec52_perm[j35] = fRec52_tmp[vsize + j35];
			}
			/* Vectorizable loop 70 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec103[i] = fZec1[i] * fZec102[i];
			}
			/* Recursive loop 71 */
			/* Pre code */
			for (int j84 = 0; j84 < 4; j84 = j84 + 1) {
				fRec103_tmp[j84] = fRec103_perm[j84];
			}
			for (int j86 = 0; j86 < 4; j86 = j86 + 1) {
				fRec104_tmp[j86] = fRec104_perm[j86];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec139[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec135[i], -0.6f), 0.35f) : fZec135[i] - 2.0f * fZec138[i] * std::max<float>(0.0f, fZec137[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec135[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec134[i] * (0.3128f - 0.010201851f * fZec136[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec137[i]))) * fZec138[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec134[i] / (fZec137[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec134[i] / std::sqrt(0.09784384f * fZec136[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec134[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec135[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec135[i], -0.5f), 0.5f) : tanhf(fZec135[i]))))) - (fConst4 * fRec103[i - 1] + fRec104[i - 1]);
				fRec103[i] = fRec103[i - 1] + fConst8 * fZec139[i];
				fZec140[i] = fRec103[i - 1] + fConst7 * fZec139[i];
				fRec104[i] = fRec104[i - 1] + fConst9 * fZec140[i];
				fZec141[i] = fConst3 * fZec140[i];
				fRec105[i] = fRec104[i - 1] + fZec141[i];
				fZec142[i] = fConst10 * fZec139[i];
				fRec106[i] = fZec142[i];
				fRec107[i] = fZec140[i];
			}
			/* Post code */
			for (int j85 = 0; j85 < 4; j85 = j85 + 1) {
				fRec103_perm[j85] = fRec103_tmp[vsize + j85];
			}
			for (int j87 = 0; j87 < 4; j87 = j87 + 1) {
				fRec104_perm[j87] = fRec104_tmp[vsize + j87];
			}
			/* Vectorizable loop 72 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec16[i] = 0.3128f * fZec15[i];
			}
			/* Vectorizable loop 73 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec30[i] = std::pow(1e+01f, 0.05f * (fRec34[i] + -2.5f));
			}
			/* Vectorizable loop 74 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec35[i] = std::pow(1e+01f, 0.05f * fRec35[i]);
			}
			/* Recursive loop 75 */
			/* Pre code */
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec47_tmp[j36] = fRec47_perm[j36];
			}
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec48_tmp[j38] = fRec48_perm[j38];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec60[i] = fRec54[i] + fRec53[i] * fZec24[i] + 1.4144272f * fRec55[i] * fZec25[i] - (fConst12 * fRec47[i - 1] + fRec48[i - 1]);
				fRec47[i] = fRec47[i - 1] + fConst15 * fZec60[i];
				fZec61[i] = fRec47[i - 1] + fConst14 * fZec60[i];
				fRec48[i] = fRec48[i - 1] + fConst16 * fZec61[i];
				fRec49[i] = fZec61[i];
				fZec62[i] = fConst17 * fZec60[i];
				fZec63[i] = fConst11 * fZec61[i];
				fRec50[i] = fZec63[i] + fRec48[i - 1] + fZec62[i];
			}
			/* Post code */
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec47_perm[j37] = fRec47_tmp[vsize + j37];
			}
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec48_perm[j39] = fRec48_tmp[vsize + j39];
			}
			/* Vectorizable loop 76 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec104[i] = 0.3128f * fZec103[i];
			}
			/* Recursive loop 77 */
			/* Pre code */
			for (int j88 = 0; j88 < 4; j88 = j88 + 1) {
				fRec99_tmp[j88] = fRec99_perm[j88];
			}
			for (int j90 = 0; j90 < 4; j90 = j90 + 1) {
				fRec100_tmp[j90] = fRec100_perm[j90];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec143[i] = fRec106[i] + fRec105[i] * fZec24[i] + 1.4144272f * fRec107[i] * fZec25[i] - (fConst12 * fRec99[i - 1] + fRec100[i - 1]);
				fRec99[i] = fRec99[i - 1] + fConst15 * fZec143[i];
				fZec144[i] = fRec99[i - 1] + fConst14 * fZec143[i];
				fRec100[i] = fRec100[i - 1] + fConst16 * fZec144[i];
				fRec101[i] = fZec144[i];
				fZec145[i] = fConst17 * fZec143[i];
				fZec146[i] = fConst11 * fZec144[i];
				fRec102[i] = fZec146[i] + fRec100[i - 1] + fZec145[i];
			}
			/* Post code */
			for (int j89 = 0; j89 < 4; j89 = j89 + 1) {
				fRec99_perm[j89] = fRec99_tmp[vsize + j89];
			}
			for (int j91 = 0; j91 < 4; j91 = j91 + 1) {
				fRec100_perm[j91] = fRec100_tmp[vsize + j91];
			}
			/* Vectorizable loop 78 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec17[i] = fZec4[i] * mlczerov_faustpower2_f(fZec14[i]);
			}
			/* Vectorizable loop 79 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec18[i] = std::fabs(fZec16[i]);
			}
			/* Vectorizable loop 80 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec19[i] = ((fZec16[i] > 0.0f) ? 1.0f : ((fZec16[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 81 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec36[i] = std::sqrt(fZec35[i]);
			}
			/* Recursive loop 82 */
			/* Pre code */
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec42_tmp[j40] = fRec42_perm[j40];
			}
			for (int j42 = 0; j42 < 4; j42 = j42 + 1) {
				fRec43_tmp[j42] = fRec43_perm[j42];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec64[i] = fRec50[i] + fRec49[i] * fZec30[i] - (fConst19 * fRec42[i - 1] + fRec43[i - 1]);
				fRec42[i] = fRec42[i - 1] + fConst22 * fZec64[i];
				fZec65[i] = fRec42[i - 1] + fConst21 * fZec64[i];
				fRec43[i] = fRec43[i - 1] + fConst23 * fZec65[i];
				fZec66[i] = fConst18 * fZec65[i];
				fRec44[i] = fRec43[i - 1] + fZec66[i];
				fZec67[i] = fConst24 * fZec64[i];
				fRec45[i] = fZec67[i];
				fRec46[i] = fZec65[i];
			}
			/* Post code */
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec42_perm[j41] = fRec42_tmp[vsize + j41];
			}
			for (int j43 = 0; j43 < 4; j43 = j43 + 1) {
				fRec43_perm[j43] = fRec43_tmp[vsize + j43];
			}
			/* Vectorizable loop 83 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec105[i] = fZec4[i] * mlczerov_faustpower2_f(fZec102[i]);
			}
			/* Vectorizable loop 84 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec106[i] = std::fabs(fZec104[i]);
			}
			/* Vectorizable loop 85 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec107[i] = ((fZec104[i] > 0.0f) ? 1.0f : ((fZec104[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 86 */
			/* Pre code */
			for (int j92 = 0; j92 < 4; j92 = j92 + 1) {
				fRec94_tmp[j92] = fRec94_perm[j92];
			}
			for (int j94 = 0; j94 < 4; j94 = j94 + 1) {
				fRec95_tmp[j94] = fRec95_perm[j94];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec147[i] = fRec102[i] + fRec101[i] * fZec30[i] - (fConst19 * fRec94[i - 1] + fRec95[i - 1]);
				fRec94[i] = fRec94[i - 1] + fConst22 * fZec147[i];
				fZec148[i] = fRec94[i - 1] + fConst21 * fZec147[i];
				fRec95[i] = fRec95[i - 1] + fConst23 * fZec148[i];
				fZec149[i] = fConst18 * fZec148[i];
				fRec96[i] = fRec95[i - 1] + fZec149[i];
				fZec150[i] = fConst24 * fZec147[i];
				fRec97[i] = fZec150[i];
				fRec98[i] = fZec148[i];
			}
			/* Post code */
			for (int j93 = 0; j93 < 4; j93 = j93 + 1) {
				fRec94_perm[j93] = fRec94_tmp[vsize + j93];
			}
			for (int j95 = 0; j95 < 4; j95 = j95 + 1) {
				fRec95_perm[j95] = fRec95_tmp[vsize + j95];
			}
			/* Recursive loop 87 */
			/* Pre code */
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec24_tmp[j10] = fRec24_perm[j10];
			}
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec25_tmp[j12] = fRec25_perm[j12];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec20[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec16[i], -0.6f), 0.35f) : fZec16[i] - 2.0f * fZec19[i] * std::max<float>(0.0f, fZec18[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec16[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec15[i] * (0.3128f - 0.010201851f * fZec17[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec18[i]))) * fZec19[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec15[i] / (fZec18[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec15[i] / std::sqrt(0.09784384f * fZec17[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec15[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec16[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec16[i], -0.5f), 0.5f) : tanhf(fZec16[i]))))) - (fConst4 * fRec24[i - 1] + fRec25[i - 1]);
				fRec24[i] = fRec24[i - 1] + fConst8 * fZec20[i];
				fZec21[i] = fRec24[i - 1] + fConst7 * fZec20[i];
				fRec25[i] = fRec25[i - 1] + fConst9 * fZec21[i];
				fZec22[i] = fConst3 * fZec21[i];
				fRec26[i] = fRec25[i - 1] + fZec22[i];
				fZec23[i] = fConst10 * fZec20[i];
				fRec27[i] = fZec23[i];
				fRec28[i] = fZec21[i];
			}
			/* Post code */
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec24_perm[j11] = fRec24_tmp[vsize + j11];
			}
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec25_perm[j13] = fRec25_tmp[vsize + j13];
			}
			/* Recursive loop 88 */
			/* Pre code */
			for (int j44 = 0; j44 < 4; j44 = j44 + 1) {
				fRec38_tmp[j44] = fRec38_perm[j44];
			}
			for (int j46 = 0; j46 < 4; j46 = j46 + 1) {
				fRec39_tmp[j46] = fRec39_perm[j46];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec68[i] = fSlow27 * (fRec44[i] + fRec45[i] * fZec35[i] + 1.4144272f * fRec46[i] * fZec36[i]) - (fConst26 * fRec38[i - 1] + fRec39[i - 1]);
				fRec38[i] = fRec38[i - 1] + fConst29 * fZec68[i];
				fZec69[i] = fRec38[i - 1] + fConst28 * fZec68[i];
				fRec39[i] = fRec39[i - 1] + fConst30 * fZec69[i];
				fRec40[i] = fZec69[i];
				fZec70[i] = fConst31 * fZec68[i];
				fZec71[i] = fConst25 * fZec69[i];
				fRec41[i] = fZec71[i] + fRec39[i - 1] + fZec70[i];
			}
			/* Post code */
			for (int j45 = 0; j45 < 4; j45 = j45 + 1) {
				fRec38_perm[j45] = fRec38_tmp[vsize + j45];
			}
			for (int j47 = 0; j47 < 4; j47 = j47 + 1) {
				fRec39_perm[j47] = fRec39_tmp[vsize + j47];
			}
			/* Recursive loop 89 */
			/* Pre code */
			for (int j68 = 0; j68 < 4; j68 = j68 + 1) {
				fRec81_tmp[j68] = fRec81_perm[j68];
			}
			for (int j70 = 0; j70 < 4; j70 = j70 + 1) {
				fRec82_tmp[j70] = fRec82_perm[j70];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec108[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec104[i], -0.6f), 0.35f) : fZec104[i] - 2.0f * fZec107[i] * std::max<float>(0.0f, fZec106[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec104[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec103[i] * (0.3128f - 0.010201851f * fZec105[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec106[i]))) * fZec107[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec103[i] / (fZec106[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec103[i] / std::sqrt(0.09784384f * fZec105[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec103[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec104[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec104[i], -0.5f), 0.5f) : tanhf(fZec104[i]))))) - (fConst4 * fRec81[i - 1] + fRec82[i - 1]);
				fRec81[i] = fRec81[i - 1] + fConst8 * fZec108[i];
				fZec109[i] = fRec81[i - 1] + fConst7 * fZec108[i];
				fRec82[i] = fRec82[i - 1] + fConst9 * fZec109[i];
				fZec110[i] = fConst3 * fZec109[i];
				fRec83[i] = fRec82[i - 1] + fZec110[i];
				fZec111[i] = fConst10 * fZec108[i];
				fRec84[i] = fZec111[i];
				fRec85[i] = fZec109[i];
			}
			/* Post code */
			for (int j69 = 0; j69 < 4; j69 = j69 + 1) {
				fRec81_perm[j69] = fRec81_tmp[vsize + j69];
			}
			for (int j71 = 0; j71 < 4; j71 = j71 + 1) {
				fRec82_perm[j71] = fRec82_tmp[vsize + j71];
			}
			/* Recursive loop 90 */
			/* Pre code */
			for (int j96 = 0; j96 < 4; j96 = j96 + 1) {
				fRec90_tmp[j96] = fRec90_perm[j96];
			}
			for (int j98 = 0; j98 < 4; j98 = j98 + 1) {
				fRec91_tmp[j98] = fRec91_perm[j98];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec151[i] = fSlow27 * (fRec96[i] + fRec97[i] * fZec35[i] + 1.4144272f * fRec98[i] * fZec36[i]) - (fConst26 * fRec90[i - 1] + fRec91[i - 1]);
				fRec90[i] = fRec90[i - 1] + fConst29 * fZec151[i];
				fZec152[i] = fRec90[i - 1] + fConst28 * fZec151[i];
				fRec91[i] = fRec91[i - 1] + fConst30 * fZec152[i];
				fRec92[i] = fZec152[i];
				fZec153[i] = fConst31 * fZec151[i];
				fZec154[i] = fConst25 * fZec152[i];
				fRec93[i] = fZec154[i] + fRec91[i - 1] + fZec153[i];
			}
			/* Post code */
			for (int j97 = 0; j97 < 4; j97 = j97 + 1) {
				fRec90_perm[j97] = fRec90_tmp[vsize + j97];
			}
			for (int j99 = 0; j99 < 4; j99 = j99 + 1) {
				fRec91_perm[j99] = fRec91_tmp[vsize + j99];
			}
			/* Recursive loop 91 */
			/* Pre code */
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec20_tmp[j16] = fRec20_perm[j16];
			}
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec21_tmp[j18] = fRec21_perm[j18];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec26[i] = fRec27[i] + fRec26[i] * fZec24[i] + 1.4144272f * fRec28[i] * fZec25[i] - (fConst12 * fRec20[i - 1] + fRec21[i - 1]);
				fRec20[i] = fRec20[i - 1] + fConst15 * fZec26[i];
				fZec27[i] = fRec20[i - 1] + fConst14 * fZec26[i];
				fRec21[i] = fRec21[i - 1] + fConst16 * fZec27[i];
				fRec22[i] = fZec27[i];
				fZec28[i] = fConst17 * fZec26[i];
				fZec29[i] = fConst11 * fZec27[i];
				fRec23[i] = fZec29[i] + fRec21[i - 1] + fZec28[i];
			}
			/* Post code */
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec20_perm[j17] = fRec20_tmp[vsize + j17];
			}
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec21_perm[j19] = fRec21_tmp[vsize + j19];
			}
			/* Vectorizable loop 92 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec72[i] = fRec41[i] + fSlow28 * fRec40[i];
			}
			/* Recursive loop 93 */
			/* Pre code */
			for (int j72 = 0; j72 < 4; j72 = j72 + 1) {
				fRec77_tmp[j72] = fRec77_perm[j72];
			}
			for (int j74 = 0; j74 < 4; j74 = j74 + 1) {
				fRec78_tmp[j74] = fRec78_perm[j74];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec112[i] = fRec84[i] + fRec83[i] * fZec24[i] + 1.4144272f * fRec85[i] * fZec25[i] - (fConst12 * fRec77[i - 1] + fRec78[i - 1]);
				fRec77[i] = fRec77[i - 1] + fConst15 * fZec112[i];
				fZec113[i] = fRec77[i - 1] + fConst14 * fZec112[i];
				fRec78[i] = fRec78[i - 1] + fConst16 * fZec113[i];
				fRec79[i] = fZec113[i];
				fZec114[i] = fConst17 * fZec112[i];
				fZec115[i] = fConst11 * fZec113[i];
				fRec80[i] = fZec115[i] + fRec78[i - 1] + fZec114[i];
			}
			/* Post code */
			for (int j73 = 0; j73 < 4; j73 = j73 + 1) {
				fRec77_perm[j73] = fRec77_tmp[vsize + j73];
			}
			for (int j75 = 0; j75 < 4; j75 = j75 + 1) {
				fRec78_perm[j75] = fRec78_tmp[vsize + j75];
			}
			/* Vectorizable loop 94 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec155[i] = fRec93[i] + fSlow28 * fRec92[i];
			}
			/* Recursive loop 95 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec15_tmp[j22] = fRec15_perm[j22];
			}
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec16_tmp[j24] = fRec16_perm[j24];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec31[i] = fRec23[i] + fRec22[i] * fZec30[i] - (fConst19 * fRec15[i - 1] + fRec16[i - 1]);
				fRec15[i] = fRec15[i - 1] + fConst22 * fZec31[i];
				fZec32[i] = fRec15[i - 1] + fConst21 * fZec31[i];
				fRec16[i] = fRec16[i - 1] + fConst23 * fZec32[i];
				fZec33[i] = fConst18 * fZec32[i];
				fRec17[i] = fRec16[i - 1] + fZec33[i];
				fZec34[i] = fConst24 * fZec31[i];
				fRec18[i] = fZec34[i];
				fRec19[i] = fZec32[i];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec15_perm[j23] = fRec15_tmp[vsize + j23];
			}
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec16_perm[j25] = fRec16_tmp[vsize + j25];
			}
			/* Vectorizable loop 96 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec73[i] = std::fabs(fZec72[i]);
			}
			/* Vectorizable loop 97 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec74[i] = ((fZec72[i] > 0.0f) ? 1.0f : ((fZec72[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 98 */
			/* Pre code */
			for (int j76 = 0; j76 < 4; j76 = j76 + 1) {
				fRec72_tmp[j76] = fRec72_perm[j76];
			}
			for (int j78 = 0; j78 < 4; j78 = j78 + 1) {
				fRec73_tmp[j78] = fRec73_perm[j78];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec116[i] = fRec80[i] + fRec79[i] * fZec30[i] - (fConst19 * fRec72[i - 1] + fRec73[i - 1]);
				fRec72[i] = fRec72[i - 1] + fConst22 * fZec116[i];
				fZec117[i] = fRec72[i - 1] + fConst21 * fZec116[i];
				fRec73[i] = fRec73[i - 1] + fConst23 * fZec117[i];
				fZec118[i] = fConst18 * fZec117[i];
				fRec74[i] = fRec73[i - 1] + fZec118[i];
				fZec119[i] = fConst24 * fZec116[i];
				fRec75[i] = fZec119[i];
				fRec76[i] = fZec117[i];
			}
			/* Post code */
			for (int j77 = 0; j77 < 4; j77 = j77 + 1) {
				fRec72_perm[j77] = fRec72_tmp[vsize + j77];
			}
			for (int j79 = 0; j79 < 4; j79 = j79 + 1) {
				fRec73_perm[j79] = fRec73_tmp[vsize + j79];
			}
			/* Vectorizable loop 99 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec156[i] = std::fabs(fZec155[i]);
			}
			/* Vectorizable loop 100 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec157[i] = ((fZec155[i] > 0.0f) ? 1.0f : ((fZec155[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 101 */
			/* Pre code */
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec11_tmp[j28] = fRec11_perm[j28];
			}
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec12_tmp[j30] = fRec12_perm[j30];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec37[i] = fSlow27 * (fRec17[i] + fRec18[i] * fZec35[i] + 1.4144272f * fRec19[i] * fZec36[i]) - (fConst26 * fRec11[i - 1] + fRec12[i - 1]);
				fRec11[i] = fRec11[i - 1] + fConst29 * fZec37[i];
				fZec38[i] = fRec11[i - 1] + fConst28 * fZec37[i];
				fRec12[i] = fRec12[i - 1] + fConst30 * fZec38[i];
				fRec13[i] = fZec38[i];
				fZec39[i] = fConst31 * fZec37[i];
				fZec40[i] = fConst25 * fZec38[i];
				fRec14[i] = fZec40[i] + fRec12[i - 1] + fZec39[i];
			}
			/* Post code */
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec11_perm[j29] = fRec11_tmp[vsize + j29];
			}
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec12_perm[j31] = fRec12_tmp[vsize + j31];
			}
			/* Vectorizable loop 102 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec75[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec72[i], -0.6f), 0.35f) : fZec72[i] - 2.0f * fZec74[i] * std::max<float>(0.0f, fZec73[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec72[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec72[i] - 0.33333334f * mlczerov_faustpower3_f(fZec72[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec73[i]))) * fZec74[i]))) : ((iSlow5) ? ((iSlow12) ? fZec72[i] / (fZec73[i] + 1.0f) : ((iSlow13) ? fZec72[i] / std::sqrt(mlczerov_faustpower2_f(fZec72[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec72[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec72[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec72[i], -0.5f), 0.5f) : tanhf(fZec72[i])))));
			}
			/* Recursive loop 103 */
			/* Pre code */
			for (int j80 = 0; j80 < 4; j80 = j80 + 1) {
				fRec68_tmp[j80] = fRec68_perm[j80];
			}
			for (int j82 = 0; j82 < 4; j82 = j82 + 1) {
				fRec69_tmp[j82] = fRec69_perm[j82];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec120[i] = fSlow27 * (fRec74[i] + fRec75[i] * fZec35[i] + 1.4144272f * fRec76[i] * fZec36[i]) - (fConst26 * fRec68[i - 1] + fRec69[i - 1]);
				fRec68[i] = fRec68[i - 1] + fConst29 * fZec120[i];
				fZec121[i] = fRec68[i - 1] + fConst28 * fZec120[i];
				fRec69[i] = fRec69[i - 1] + fConst30 * fZec121[i];
				fRec70[i] = fZec121[i];
				fZec122[i] = fConst31 * fZec120[i];
				fZec123[i] = fConst25 * fZec121[i];
				fRec71[i] = fZec123[i] + fRec69[i - 1] + fZec122[i];
			}
			/* Post code */
			for (int j81 = 0; j81 < 4; j81 = j81 + 1) {
				fRec68_perm[j81] = fRec68_tmp[vsize + j81];
			}
			for (int j83 = 0; j83 < 4; j83 = j83 + 1) {
				fRec69_perm[j83] = fRec69_tmp[vsize + j83];
			}
			/* Vectorizable loop 104 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec158[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec155[i], -0.6f), 0.35f) : fZec155[i] - 2.0f * fZec157[i] * std::max<float>(0.0f, fZec156[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec155[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec155[i] - 0.33333334f * mlczerov_faustpower3_f(fZec155[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec156[i]))) * fZec157[i]))) : ((iSlow5) ? ((iSlow12) ? fZec155[i] / (fZec156[i] + 1.0f) : ((iSlow13) ? fZec155[i] / std::sqrt(mlczerov_faustpower2_f(fZec155[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec155[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec155[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec155[i], -0.5f), 0.5f) : tanhf(fZec155[i])))));
			}
			/* Recursive loop 105 */
			/* Pre code */
			for (int j48 = 0; j48 < 4; j48 = j48 + 1) {
				fRec37_tmp[j48] = fRec37_perm[j48];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec37[i] = std::max<float>(0.995f * fRec37[i - 1], std::fabs(fSlow29 * fZec75[i]));
			}
			/* Post code */
			for (int j49 = 0; j49 < 4; j49 = j49 + 1) {
				fRec37_perm[j49] = fRec37_tmp[vsize + j49];
			}
			/* Vectorizable loop 106 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec76[i] = fRec14[i] + fSlow28 * fRec13[i];
			}
			/* Recursive loop 107 */
			/* Pre code */
			for (int j56 = 0; j56 < 4; j56 = j56 + 1) {
				fRec56_tmp[j56] = fRec56_perm[j56];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec56[i] = fSlow31 + fConst2 * fRec56[i - 1];
			}
			/* Post code */
			for (int j57 = 0; j57 < 4; j57 = j57 + 1) {
				fRec56_perm[j57] = fRec56_tmp[vsize + j57];
			}
			/* Recursive loop 108 */
			/* Pre code */
			for (int j100 = 0; j100 < 4; j100 = j100 + 1) {
				fRec89_tmp[j100] = fRec89_perm[j100];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec89[i] = std::max<float>(0.995f * fRec89[i - 1], std::fabs(fSlow29 * fZec158[i]));
			}
			/* Post code */
			for (int j101 = 0; j101 < 4; j101 = j101 + 1) {
				fRec89_perm[j101] = fRec89_tmp[vsize + j101];
			}
			/* Vectorizable loop 109 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec159[i] = fRec71[i] + fSlow28 * fRec70[i];
			}
			/* Recursive loop 110 */
			/* Pre code */
			for (int j50 = 0; j50 < 4; j50 = j50 + 1) {
				fRec36_tmp[j50] = fRec36_perm[j50];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec36[i] = fConst1 * static_cast<float>(fRec37[i] > fZec0[i]) + fConst2 * fRec36[i - 1];
			}
			/* Post code */
			for (int j51 = 0; j51 < 4; j51 = j51 + 1) {
				fRec36_perm[j51] = fRec36_tmp[vsize + j51];
			}
			/* Vectorizable loop 111 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec77[i] = std::fabs(fZec76[i]);
			}
			/* Vectorizable loop 112 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec78[i] = ((fZec76[i] > 0.0f) ? 1.0f : ((fZec76[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 113 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec83[i] = std::pow(1e+01f, fSlow33 * fRec56[i]);
			}
			/* Recursive loop 114 */
			/* Pre code */
			for (int j62 = 0; j62 < 4; j62 = j62 + 1) {
				fRec57_tmp[j62] = fRec57_perm[j62];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec57[i] = fSlow34 + fConst2 * fRec57[i - 1];
			}
			/* Post code */
			for (int j63 = 0; j63 < 4; j63 = j63 + 1) {
				fRec57_perm[j63] = fRec57_tmp[vsize + j63];
			}
			/* Recursive loop 115 */
			/* Pre code */
			for (int j102 = 0; j102 < 4; j102 = j102 + 1) {
				fRec88_tmp[j102] = fRec88_perm[j102];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec88[i] = fConst1 * static_cast<float>(fRec89[i] > fZec0[i]) + fConst2 * fRec88[i - 1];
			}
			/* Post code */
			for (int j103 = 0; j103 < 4; j103 = j103 + 1) {
				fRec88_perm[j103] = fRec88_tmp[vsize + j103];
			}
			/* Vectorizable loop 116 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec160[i] = std::fabs(fZec159[i]);
			}
			/* Vectorizable loop 117 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec161[i] = ((fZec159[i] > 0.0f) ? 1.0f : ((fZec159[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 118 */
			/* Pre code */
			for (int j52 = 0; j52 < 4; j52 = j52 + 1) {
				fRec6_tmp[j52] = fRec6_perm[j52];
			}
			for (int j54 = 0; j54 < 4; j54 = j54 + 1) {
				fRec7_tmp[j54] = fRec7_perm[j54];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec79[i] = ((iSlow30) ? fSlow29 * fRec36[i] * fZec75[i] : fSlow29 * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec76[i], -0.6f), 0.35f) : fZec76[i] - 2.0f * fZec78[i] * std::max<float>(0.0f, fZec77[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec76[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec76[i] - 0.33333334f * mlczerov_faustpower3_f(fZec76[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec77[i]))) * fZec78[i]))) : ((iSlow5) ? ((iSlow12) ? fZec76[i] / (fZec77[i] + 1.0f) : ((iSlow13) ? fZec76[i] / std::sqrt(mlczerov_faustpower2_f(fZec76[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec76[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec76[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec76[i], -0.5f), 0.5f) : tanhf(fZec76[i])))))) - (fConst33 * fRec6[i - 1] + fRec7[i - 1]);
				fRec6[i] = fRec6[i - 1] + fConst36 * fZec79[i];
				fZec80[i] = fRec6[i - 1] + fConst35 * fZec79[i];
				fRec7[i] = fRec7[i - 1] + fConst37 * fZec80[i];
				fZec81[i] = fConst32 * fZec80[i];
				fRec8[i] = fRec7[i - 1] + fZec81[i];
				fZec82[i] = fConst38 * fZec79[i];
				fRec9[i] = fZec82[i];
				fRec10[i] = fZec80[i];
			}
			/* Post code */
			for (int j53 = 0; j53 < 4; j53 = j53 + 1) {
				fRec6_perm[j53] = fRec6_tmp[vsize + j53];
			}
			for (int j55 = 0; j55 < 4; j55 = j55 + 1) {
				fRec7_perm[j55] = fRec7_tmp[vsize + j55];
			}
			/* Vectorizable loop 119 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec84[i] = std::sqrt(fZec83[i]);
			}
			/* Vectorizable loop 120 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec89[i] = std::pow(1e+01f, fSlow35 * fRec57[i]);
			}
			/* Recursive loop 121 */
			/* Pre code */
			for (int j104 = 0; j104 < 4; j104 = j104 + 1) {
				fRec63_tmp[j104] = fRec63_perm[j104];
			}
			for (int j106 = 0; j106 < 4; j106 = j106 + 1) {
				fRec64_tmp[j106] = fRec64_perm[j106];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec162[i] = ((iSlow30) ? fSlow29 * fRec88[i] * fZec158[i] : fSlow29 * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec159[i], -0.6f), 0.35f) : fZec159[i] - 2.0f * fZec161[i] * std::max<float>(0.0f, fZec160[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec159[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec159[i] - 0.33333334f * mlczerov_faustpower3_f(fZec159[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec160[i]))) * fZec161[i]))) : ((iSlow5) ? ((iSlow12) ? fZec159[i] / (fZec160[i] + 1.0f) : ((iSlow13) ? fZec159[i] / std::sqrt(mlczerov_faustpower2_f(fZec159[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec159[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec159[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec159[i], -0.5f), 0.5f) : tanhf(fZec159[i])))))) - (fConst33 * fRec63[i - 1] + fRec64[i - 1]);
				fRec63[i] = fRec63[i - 1] + fConst36 * fZec162[i];
				fZec163[i] = fRec63[i - 1] + fConst35 * fZec162[i];
				fRec64[i] = fRec64[i - 1] + fConst37 * fZec163[i];
				fZec164[i] = fConst32 * fZec163[i];
				fRec65[i] = fRec64[i - 1] + fZec164[i];
				fZec165[i] = fConst38 * fZec162[i];
				fRec66[i] = fZec165[i];
				fRec67[i] = fZec163[i];
			}
			/* Post code */
			for (int j105 = 0; j105 < 4; j105 = j105 + 1) {
				fRec63_perm[j105] = fRec63_tmp[vsize + j105];
			}
			for (int j107 = 0; j107 < 4; j107 = j107 + 1) {
				fRec64_perm[j107] = fRec64_tmp[vsize + j107];
			}
			/* Recursive loop 122 */
			/* Pre code */
			for (int j58 = 0; j58 < 4; j58 = j58 + 1) {
				fRec1_tmp[j58] = fRec1_perm[j58];
			}
			for (int j60 = 0; j60 < 4; j60 = j60 + 1) {
				fRec2_tmp[j60] = fRec2_perm[j60];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec85[i] = fRec9[i] + fRec8[i] * fZec83[i] + 1.25f * fRec10[i] * fZec84[i] - (fConst40 * fRec1[i - 1] + fRec2[i - 1]);
				fRec1[i] = fRec1[i - 1] + fConst43 * fZec85[i];
				fZec86[i] = fRec1[i - 1] + fConst42 * fZec85[i];
				fRec2[i] = fRec2[i - 1] + fConst44 * fZec86[i];
				fZec87[i] = fConst39 * fZec86[i];
				fRec3[i] = fRec2[i - 1] + fZec87[i];
				fZec88[i] = fConst45 * fZec85[i];
				fRec4[i] = fZec88[i];
				fRec5[i] = fZec86[i];
			}
			/* Post code */
			for (int j59 = 0; j59 < 4; j59 = j59 + 1) {
				fRec1_perm[j59] = fRec1_tmp[vsize + j59];
			}
			for (int j61 = 0; j61 < 4; j61 = j61 + 1) {
				fRec2_perm[j61] = fRec2_tmp[vsize + j61];
			}
			/* Recursive loop 123 */
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
			/* Vectorizable loop 124 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec90[i] = std::sqrt(fZec89[i]);
			}
			/* Recursive loop 125 */
			/* Pre code */
			for (int j108 = 0; j108 < 4; j108 = j108 + 1) {
				fRec58_tmp[j108] = fRec58_perm[j108];
			}
			for (int j110 = 0; j110 < 4; j110 = j110 + 1) {
				fRec59_tmp[j110] = fRec59_perm[j110];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec166[i] = fRec66[i] + fRec65[i] * fZec83[i] + 1.25f * fRec67[i] * fZec84[i] - (fConst40 * fRec58[i - 1] + fRec59[i - 1]);
				fRec58[i] = fRec58[i - 1] + fConst43 * fZec166[i];
				fZec167[i] = fRec58[i - 1] + fConst42 * fZec166[i];
				fRec59[i] = fRec59[i - 1] + fConst44 * fZec167[i];
				fZec168[i] = fConst39 * fZec167[i];
				fRec60[i] = fRec59[i - 1] + fZec168[i];
				fZec169[i] = fConst45 * fZec166[i];
				fRec61[i] = fZec169[i];
				fRec62[i] = fZec167[i];
			}
			/* Post code */
			for (int j109 = 0; j109 < 4; j109 = j109 + 1) {
				fRec58_perm[j109] = fRec58_tmp[vsize + j109];
			}
			for (int j111 = 0; j111 < 4; j111 = j111 + 1) {
				fRec59_perm[j111] = fRec59_tmp[vsize + j111];
			}
			/* Vectorizable loop 126 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec3[i] + fRec4[i] * fZec89[i] + 1.4285715f * fRec5[i] * fZec90[i]));
			}
			/* Vectorizable loop 127 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec60[i] + fRec61[i] * fZec89[i] + 1.4285715f * fRec62[i] * fZec90[i]));
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
				fZec1[i] = 72.0f * fRec29[i] + 8.0f;
			}
			/* Vectorizable loop 2 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec91[i] = static_cast<float>(input1[i]) * fRec29[i];
			}
			/* Recursive loop 3 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec32_tmp[j6] = fRec32_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec32[i] = fSlow2 + fConst2 * fRec32[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec32_perm[j7] = fRec32_tmp[vsize + j7];
			}
			/* Vectorizable loop 4 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec5[i] = mlczerov_faustpower2_f(fRec29[i]);
			}
			/* Vectorizable loop 5 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec41[i] = static_cast<float>(input0[i]) * fRec29[i] * fZec1[i];
			}
			/* Vectorizable loop 6 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec124[i] = fZec91[i] * fZec1[i];
			}
			/* Recursive loop 7 */
			/* Pre code */
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec31_tmp[j4] = fRec31_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec31[i] = std::max<float>(0.995f * fRec31[i - 1], std::fabs(static_cast<float>(input0[i])));
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec31_perm[j5] = fRec31_tmp[vsize + j5];
			}
			/* Vectorizable loop 8 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::pow(1e+01f, 0.05f * fRec32[i]);
			}
			/* Vectorizable loop 9 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec4[i] = mlczerov_faustpower2_f(fZec1[i]);
			}
			/* Vectorizable loop 10 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec6[i] = mlczerov_faustpower2_f(static_cast<float>(input0[i]));
			}
			/* Vectorizable loop 11 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec42[i] = fSlow11 * fZec41[i];
			}
			/* Recursive loop 12 */
			/* Pre code */
			for (int j64 = 0; j64 < 4; j64 = j64 + 1) {
				fRec87_tmp[j64] = fRec87_perm[j64];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec87[i] = std::max<float>(0.995f * fRec87[i - 1], std::fabs(static_cast<float>(input1[i])));
			}
			/* Post code */
			for (int j65 = 0; j65 < 4; j65 = j65 + 1) {
				fRec87_perm[j65] = fRec87_tmp[vsize + j65];
			}
			/* Vectorizable loop 13 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec94[i] = mlczerov_faustpower2_f(static_cast<float>(input1[i])) * fZec5[i];
			}
			/* Vectorizable loop 14 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec125[i] = fSlow11 * fZec124[i];
			}
			/* Recursive loop 15 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec30_tmp[j8] = fRec30_perm[j8];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec30[i] = fConst1 * static_cast<float>(fRec31[i] > fZec0[i]) + fConst2 * fRec30[i - 1];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec30_perm[j9] = fRec30_tmp[vsize + j9];
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec43[i] = fZec6[i] * fZec5[i] * fZec4[i];
			}
			/* Vectorizable loop 17 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec44[i] = std::fabs(fZec42[i]);
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec45[i] = ((fZec42[i] > 0.0f) ? 1.0f : ((fZec42[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 19 */
			/* Pre code */
			for (int j66 = 0; j66 < 4; j66 = j66 + 1) {
				fRec86_tmp[j66] = fRec86_perm[j66];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec86[i] = fConst1 * static_cast<float>(fRec87[i] > fZec0[i]) + fConst2 * fRec86[i - 1];
			}
			/* Post code */
			for (int j67 = 0; j67 < 4; j67 = j67 + 1) {
				fRec86_perm[j67] = fRec86_tmp[vsize + j67];
			}
			/* Vectorizable loop 20 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec126[i] = fZec94[i] * fZec4[i];
			}
			/* Vectorizable loop 21 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec127[i] = std::fabs(fZec125[i]);
			}
			/* Vectorizable loop 22 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec128[i] = ((fZec125[i] > 0.0f) ? 1.0f : ((fZec125[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec2[i] = static_cast<float>(input0[i]) * fRec30[i] * fRec29[i] * fZec1[i];
			}
			/* Vectorizable loop 24 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec46[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec42[i], -0.6f), 0.35f) : fZec42[i] - 2.0f * fZec45[i] * std::max<float>(0.0f, fZec44[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec42[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec41[i] * (0.22f - fSlow20 * fZec43[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec44[i]))) * fZec45[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec41[i] / (fZec44[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec41[i] / std::sqrt(fSlow16 * fZec43[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec41[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec42[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec42[i], -0.5f), 0.5f) : tanhf(fZec42[i])))));
			}
			/* Vectorizable loop 25 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec92[i] = fZec91[i] * fRec86[i] * fZec1[i];
			}
			/* Vectorizable loop 26 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec129[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec125[i], -0.6f), 0.35f) : fZec125[i] - 2.0f * fZec128[i] * std::max<float>(0.0f, fZec127[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec125[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec124[i] * (0.22f - fSlow20 * fZec126[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec127[i]))) * fZec128[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec124[i] / (fZec127[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec124[i] / std::sqrt(fSlow16 * fZec126[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec124[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec125[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec125[i], -0.5f), 0.5f) : tanhf(fZec125[i])))));
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec3[i] = fSlow11 * fZec2[i];
			}
			/* Vectorizable loop 28 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec47[i] = fZec46[i] + 0.03f;
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec93[i] = fSlow11 * fZec92[i];
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec130[i] = fZec129[i] + 0.03f;
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec7[i] = fZec6[i] * mlczerov_faustpower2_f(fRec30[i]) * fZec5[i] * fZec4[i];
			}
			/* Vectorizable loop 32 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec8[i] = std::fabs(fZec3[i]);
			}
			/* Vectorizable loop 33 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = ((fZec3[i] > 0.0f) ? 1.0f : ((fZec3[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 34 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec48[i] = std::fabs(fZec47[i]);
			}
			/* Vectorizable loop 35 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec49[i] = ((fZec47[i] > 0.0f) ? 1.0f : ((fZec47[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 36 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec95[i] = fZec94[i] * mlczerov_faustpower2_f(fRec86[i]) * fZec4[i];
			}
			/* Vectorizable loop 37 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec96[i] = std::fabs(fZec93[i]);
			}
			/* Vectorizable loop 38 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec97[i] = ((fZec93[i] > 0.0f) ? 1.0f : ((fZec93[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 39 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec131[i] = std::fabs(fZec130[i]);
			}
			/* Vectorizable loop 40 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec132[i] = ((fZec130[i] > 0.0f) ? 1.0f : ((fZec130[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 41 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec3[i], -0.6f), 0.35f) : fZec3[i] - 2.0f * fZec9[i] * std::max<float>(0.0f, fZec8[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec3[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec2[i] * (0.22f - fSlow20 * fZec7[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec8[i]))) * fZec9[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec2[i] / (fZec8[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec2[i] / std::sqrt(fSlow16 * fZec7[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec2[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec3[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec3[i], -0.5f), 0.5f) : tanhf(fZec3[i])))));
			}
			/* Vectorizable loop 42 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec50[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec47[i], -0.6f), 0.35f) : fZec46[i] + (0.03f - 2.0f * fZec49[i] * std::max<float>(0.0f, fZec48[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec46[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec46[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec47[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec48[i]))) * fZec49[i]))) : ((iSlow5) ? ((iSlow12) ? fZec47[i] / (fZec48[i] + 1.0f) : ((iSlow13) ? fZec47[i] / std::sqrt(mlczerov_faustpower2_f(fZec47[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec47[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec47[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec47[i], -0.5f), 0.5f) : tanhf(fZec47[i])))));
			}
			/* Vectorizable loop 43 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec98[i] = fSlow22 * fZec1[i] * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec93[i], -0.6f), 0.35f) : fZec93[i] - 2.0f * fZec97[i] * std::max<float>(0.0f, fZec96[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec93[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fSlow10 * fZec92[i] * (0.22f - fSlow20 * fZec95[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec96[i]))) * fZec97[i]))) : ((iSlow5) ? ((iSlow12) ? fSlow11 * (fZec92[i] / (fZec96[i] + 1.0f)) : ((iSlow13) ? fSlow11 * (fZec92[i] / std::sqrt(fSlow16 * fZec95[i] + 1.0f)) : 0.63661975f * std::atan(fSlow14 * fZec92[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec93[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec93[i], -0.5f), 0.5f) : tanhf(fZec93[i])))));
			}
			/* Vectorizable loop 44 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec133[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec130[i], -0.6f), 0.35f) : fZec129[i] + (0.03f - 2.0f * fZec132[i] * std::max<float>(0.0f, fZec131[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec129[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec129[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec130[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec131[i]))) * fZec132[i]))) : ((iSlow5) ? ((iSlow12) ? fZec130[i] / (fZec131[i] + 1.0f) : ((iSlow13) ? fZec130[i] / std::sqrt(mlczerov_faustpower2_f(fZec130[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec130[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec130[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec130[i], -0.5f), 0.5f) : tanhf(fZec130[i])))));
			}
			/* Vectorizable loop 45 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = fZec10[i] + 0.03f;
			}
			/* Vectorizable loop 46 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec51[i] = fZec1[i] * fZec50[i];
			}
			/* Vectorizable loop 47 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec99[i] = fZec98[i] + 0.03f;
			}
			/* Vectorizable loop 48 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec134[i] = fZec1[i] * fZec133[i];
			}
			/* Vectorizable loop 49 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = std::fabs(fZec11[i]);
			}
			/* Vectorizable loop 50 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec13[i] = ((fZec11[i] > 0.0f) ? 1.0f : ((fZec11[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 51 */
			/* Pre code */
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec33_tmp[j14] = fRec33_perm[j14];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec33[i] = fSlow23 + fConst2 * fRec33[i - 1];
			}
			/* Post code */
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec33_perm[j15] = fRec33_tmp[vsize + j15];
			}
			/* Vectorizable loop 52 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec52[i] = 0.3128f * fZec51[i];
			}
			/* Vectorizable loop 53 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec100[i] = std::fabs(fZec99[i]);
			}
			/* Vectorizable loop 54 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec101[i] = ((fZec99[i] > 0.0f) ? 1.0f : ((fZec99[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 55 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec135[i] = 0.3128f * fZec134[i];
			}
			/* Vectorizable loop 56 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec14[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec11[i], -0.6f), 0.35f) : fZec10[i] + (0.03f - 2.0f * fZec13[i] * std::max<float>(0.0f, fZec12[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec10[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec10[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec11[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec12[i]))) * fZec13[i]))) : ((iSlow5) ? ((iSlow12) ? fZec11[i] / (fZec12[i] + 1.0f) : ((iSlow13) ? fZec11[i] / std::sqrt(mlczerov_faustpower2_f(fZec11[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec11[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec11[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec11[i], -0.5f), 0.5f) : tanhf(fZec11[i])))));
			}
			/* Vectorizable loop 57 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec24[i] = std::pow(1e+01f, 0.05f * fRec33[i]);
			}
			/* Vectorizable loop 58 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec53[i] = fZec4[i] * mlczerov_faustpower2_f(fZec50[i]);
			}
			/* Vectorizable loop 59 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec54[i] = std::fabs(fZec52[i]);
			}
			/* Vectorizable loop 60 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec55[i] = ((fZec52[i] > 0.0f) ? 1.0f : ((fZec52[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 61 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec102[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec99[i], -0.6f), 0.35f) : fZec98[i] + (0.03f - 2.0f * fZec101[i] * std::max<float>(0.0f, fZec100[i] + -0.6f))) : ((iSlow18) ? tanhf(fZec98[i] + 0.28f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec98[i] + (0.03f - 0.33333334f * mlczerov_faustpower3_f(fZec99[i])), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec100[i]))) * fZec101[i]))) : ((iSlow5) ? ((iSlow12) ? fZec99[i] / (fZec100[i] + 1.0f) : ((iSlow13) ? fZec99[i] / std::sqrt(mlczerov_faustpower2_f(fZec99[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec99[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec99[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec99[i], -0.5f), 0.5f) : tanhf(fZec99[i])))));
			}
			/* Vectorizable loop 62 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec136[i] = fZec4[i] * mlczerov_faustpower2_f(fZec133[i]);
			}
			/* Vectorizable loop 63 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec137[i] = std::fabs(fZec135[i]);
			}
			/* Vectorizable loop 64 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec138[i] = ((fZec135[i] > 0.0f) ? 1.0f : ((fZec135[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 65 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec15[i] = fZec1[i] * fZec14[i];
			}
			/* Vectorizable loop 66 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec25[i] = std::sqrt(fZec24[i]);
			}
			/* Recursive loop 67 */
			/* Pre code */
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec34_tmp[j20] = fRec34_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec34[i] = fSlow24 + fConst2 * fRec34[i - 1];
			}
			/* Post code */
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec34_perm[j21] = fRec34_tmp[vsize + j21];
			}
			/* Recursive loop 68 */
			/* Pre code */
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec35_tmp[j26] = fRec35_perm[j26];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec35[i] = fSlow25 + fConst2 * fRec35[i - 1];
			}
			/* Post code */
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec35_perm[j27] = fRec35_tmp[vsize + j27];
			}
			/* Recursive loop 69 */
			/* Pre code */
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec51_tmp[j32] = fRec51_perm[j32];
			}
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec52_tmp[j34] = fRec52_perm[j34];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec56[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec52[i], -0.6f), 0.35f) : fZec52[i] - 2.0f * fZec55[i] * std::max<float>(0.0f, fZec54[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec52[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec51[i] * (0.3128f - 0.010201851f * fZec53[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec54[i]))) * fZec55[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec51[i] / (fZec54[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec51[i] / std::sqrt(0.09784384f * fZec53[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec51[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec52[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec52[i], -0.5f), 0.5f) : tanhf(fZec52[i]))))) - (fConst4 * fRec51[i - 1] + fRec52[i - 1]);
				fRec51[i] = fRec51[i - 1] + fConst8 * fZec56[i];
				fZec57[i] = fRec51[i - 1] + fConst7 * fZec56[i];
				fRec52[i] = fRec52[i - 1] + fConst9 * fZec57[i];
				fZec58[i] = fConst3 * fZec57[i];
				fRec53[i] = fRec52[i - 1] + fZec58[i];
				fZec59[i] = fConst10 * fZec56[i];
				fRec54[i] = fZec59[i];
				fRec55[i] = fZec57[i];
			}
			/* Post code */
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec51_perm[j33] = fRec51_tmp[vsize + j33];
			}
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec52_perm[j35] = fRec52_tmp[vsize + j35];
			}
			/* Vectorizable loop 70 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec103[i] = fZec1[i] * fZec102[i];
			}
			/* Recursive loop 71 */
			/* Pre code */
			for (int j84 = 0; j84 < 4; j84 = j84 + 1) {
				fRec103_tmp[j84] = fRec103_perm[j84];
			}
			for (int j86 = 0; j86 < 4; j86 = j86 + 1) {
				fRec104_tmp[j86] = fRec104_perm[j86];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec139[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec135[i], -0.6f), 0.35f) : fZec135[i] - 2.0f * fZec138[i] * std::max<float>(0.0f, fZec137[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec135[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec134[i] * (0.3128f - 0.010201851f * fZec136[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec137[i]))) * fZec138[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec134[i] / (fZec137[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec134[i] / std::sqrt(0.09784384f * fZec136[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec134[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec135[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec135[i], -0.5f), 0.5f) : tanhf(fZec135[i]))))) - (fConst4 * fRec103[i - 1] + fRec104[i - 1]);
				fRec103[i] = fRec103[i - 1] + fConst8 * fZec139[i];
				fZec140[i] = fRec103[i - 1] + fConst7 * fZec139[i];
				fRec104[i] = fRec104[i - 1] + fConst9 * fZec140[i];
				fZec141[i] = fConst3 * fZec140[i];
				fRec105[i] = fRec104[i - 1] + fZec141[i];
				fZec142[i] = fConst10 * fZec139[i];
				fRec106[i] = fZec142[i];
				fRec107[i] = fZec140[i];
			}
			/* Post code */
			for (int j85 = 0; j85 < 4; j85 = j85 + 1) {
				fRec103_perm[j85] = fRec103_tmp[vsize + j85];
			}
			for (int j87 = 0; j87 < 4; j87 = j87 + 1) {
				fRec104_perm[j87] = fRec104_tmp[vsize + j87];
			}
			/* Vectorizable loop 72 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec16[i] = 0.3128f * fZec15[i];
			}
			/* Vectorizable loop 73 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec30[i] = std::pow(1e+01f, 0.05f * (fRec34[i] + -2.5f));
			}
			/* Vectorizable loop 74 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec35[i] = std::pow(1e+01f, 0.05f * fRec35[i]);
			}
			/* Recursive loop 75 */
			/* Pre code */
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec47_tmp[j36] = fRec47_perm[j36];
			}
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec48_tmp[j38] = fRec48_perm[j38];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec60[i] = fRec54[i] + fRec53[i] * fZec24[i] + 1.4144272f * fRec55[i] * fZec25[i] - (fConst12 * fRec47[i - 1] + fRec48[i - 1]);
				fRec47[i] = fRec47[i - 1] + fConst15 * fZec60[i];
				fZec61[i] = fRec47[i - 1] + fConst14 * fZec60[i];
				fRec48[i] = fRec48[i - 1] + fConst16 * fZec61[i];
				fRec49[i] = fZec61[i];
				fZec62[i] = fConst17 * fZec60[i];
				fZec63[i] = fConst11 * fZec61[i];
				fRec50[i] = fZec63[i] + fRec48[i - 1] + fZec62[i];
			}
			/* Post code */
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec47_perm[j37] = fRec47_tmp[vsize + j37];
			}
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec48_perm[j39] = fRec48_tmp[vsize + j39];
			}
			/* Vectorizable loop 76 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec104[i] = 0.3128f * fZec103[i];
			}
			/* Recursive loop 77 */
			/* Pre code */
			for (int j88 = 0; j88 < 4; j88 = j88 + 1) {
				fRec99_tmp[j88] = fRec99_perm[j88];
			}
			for (int j90 = 0; j90 < 4; j90 = j90 + 1) {
				fRec100_tmp[j90] = fRec100_perm[j90];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec143[i] = fRec106[i] + fRec105[i] * fZec24[i] + 1.4144272f * fRec107[i] * fZec25[i] - (fConst12 * fRec99[i - 1] + fRec100[i - 1]);
				fRec99[i] = fRec99[i - 1] + fConst15 * fZec143[i];
				fZec144[i] = fRec99[i - 1] + fConst14 * fZec143[i];
				fRec100[i] = fRec100[i - 1] + fConst16 * fZec144[i];
				fRec101[i] = fZec144[i];
				fZec145[i] = fConst17 * fZec143[i];
				fZec146[i] = fConst11 * fZec144[i];
				fRec102[i] = fZec146[i] + fRec100[i - 1] + fZec145[i];
			}
			/* Post code */
			for (int j89 = 0; j89 < 4; j89 = j89 + 1) {
				fRec99_perm[j89] = fRec99_tmp[vsize + j89];
			}
			for (int j91 = 0; j91 < 4; j91 = j91 + 1) {
				fRec100_perm[j91] = fRec100_tmp[vsize + j91];
			}
			/* Vectorizable loop 78 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec17[i] = fZec4[i] * mlczerov_faustpower2_f(fZec14[i]);
			}
			/* Vectorizable loop 79 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec18[i] = std::fabs(fZec16[i]);
			}
			/* Vectorizable loop 80 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec19[i] = ((fZec16[i] > 0.0f) ? 1.0f : ((fZec16[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 81 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec36[i] = std::sqrt(fZec35[i]);
			}
			/* Recursive loop 82 */
			/* Pre code */
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec42_tmp[j40] = fRec42_perm[j40];
			}
			for (int j42 = 0; j42 < 4; j42 = j42 + 1) {
				fRec43_tmp[j42] = fRec43_perm[j42];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec64[i] = fRec50[i] + fRec49[i] * fZec30[i] - (fConst19 * fRec42[i - 1] + fRec43[i - 1]);
				fRec42[i] = fRec42[i - 1] + fConst22 * fZec64[i];
				fZec65[i] = fRec42[i - 1] + fConst21 * fZec64[i];
				fRec43[i] = fRec43[i - 1] + fConst23 * fZec65[i];
				fZec66[i] = fConst18 * fZec65[i];
				fRec44[i] = fRec43[i - 1] + fZec66[i];
				fZec67[i] = fConst24 * fZec64[i];
				fRec45[i] = fZec67[i];
				fRec46[i] = fZec65[i];
			}
			/* Post code */
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec42_perm[j41] = fRec42_tmp[vsize + j41];
			}
			for (int j43 = 0; j43 < 4; j43 = j43 + 1) {
				fRec43_perm[j43] = fRec43_tmp[vsize + j43];
			}
			/* Vectorizable loop 83 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec105[i] = fZec4[i] * mlczerov_faustpower2_f(fZec102[i]);
			}
			/* Vectorizable loop 84 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec106[i] = std::fabs(fZec104[i]);
			}
			/* Vectorizable loop 85 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec107[i] = ((fZec104[i] > 0.0f) ? 1.0f : ((fZec104[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 86 */
			/* Pre code */
			for (int j92 = 0; j92 < 4; j92 = j92 + 1) {
				fRec94_tmp[j92] = fRec94_perm[j92];
			}
			for (int j94 = 0; j94 < 4; j94 = j94 + 1) {
				fRec95_tmp[j94] = fRec95_perm[j94];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec147[i] = fRec102[i] + fRec101[i] * fZec30[i] - (fConst19 * fRec94[i - 1] + fRec95[i - 1]);
				fRec94[i] = fRec94[i - 1] + fConst22 * fZec147[i];
				fZec148[i] = fRec94[i - 1] + fConst21 * fZec147[i];
				fRec95[i] = fRec95[i - 1] + fConst23 * fZec148[i];
				fZec149[i] = fConst18 * fZec148[i];
				fRec96[i] = fRec95[i - 1] + fZec149[i];
				fZec150[i] = fConst24 * fZec147[i];
				fRec97[i] = fZec150[i];
				fRec98[i] = fZec148[i];
			}
			/* Post code */
			for (int j93 = 0; j93 < 4; j93 = j93 + 1) {
				fRec94_perm[j93] = fRec94_tmp[vsize + j93];
			}
			for (int j95 = 0; j95 < 4; j95 = j95 + 1) {
				fRec95_perm[j95] = fRec95_tmp[vsize + j95];
			}
			/* Recursive loop 87 */
			/* Pre code */
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec24_tmp[j10] = fRec24_perm[j10];
			}
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec25_tmp[j12] = fRec25_perm[j12];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec20[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec16[i], -0.6f), 0.35f) : fZec16[i] - 2.0f * fZec19[i] * std::max<float>(0.0f, fZec18[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec16[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec15[i] * (0.3128f - 0.010201851f * fZec17[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec18[i]))) * fZec19[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec15[i] / (fZec18[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec15[i] / std::sqrt(0.09784384f * fZec17[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec15[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec16[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec16[i], -0.5f), 0.5f) : tanhf(fZec16[i]))))) - (fConst4 * fRec24[i - 1] + fRec25[i - 1]);
				fRec24[i] = fRec24[i - 1] + fConst8 * fZec20[i];
				fZec21[i] = fRec24[i - 1] + fConst7 * fZec20[i];
				fRec25[i] = fRec25[i - 1] + fConst9 * fZec21[i];
				fZec22[i] = fConst3 * fZec21[i];
				fRec26[i] = fRec25[i - 1] + fZec22[i];
				fZec23[i] = fConst10 * fZec20[i];
				fRec27[i] = fZec23[i];
				fRec28[i] = fZec21[i];
			}
			/* Post code */
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec24_perm[j11] = fRec24_tmp[vsize + j11];
			}
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec25_perm[j13] = fRec25_tmp[vsize + j13];
			}
			/* Recursive loop 88 */
			/* Pre code */
			for (int j44 = 0; j44 < 4; j44 = j44 + 1) {
				fRec38_tmp[j44] = fRec38_perm[j44];
			}
			for (int j46 = 0; j46 < 4; j46 = j46 + 1) {
				fRec39_tmp[j46] = fRec39_perm[j46];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec68[i] = fSlow27 * (fRec44[i] + fRec45[i] * fZec35[i] + 1.4144272f * fRec46[i] * fZec36[i]) - (fConst26 * fRec38[i - 1] + fRec39[i - 1]);
				fRec38[i] = fRec38[i - 1] + fConst29 * fZec68[i];
				fZec69[i] = fRec38[i - 1] + fConst28 * fZec68[i];
				fRec39[i] = fRec39[i - 1] + fConst30 * fZec69[i];
				fRec40[i] = fZec69[i];
				fZec70[i] = fConst31 * fZec68[i];
				fZec71[i] = fConst25 * fZec69[i];
				fRec41[i] = fZec71[i] + fRec39[i - 1] + fZec70[i];
			}
			/* Post code */
			for (int j45 = 0; j45 < 4; j45 = j45 + 1) {
				fRec38_perm[j45] = fRec38_tmp[vsize + j45];
			}
			for (int j47 = 0; j47 < 4; j47 = j47 + 1) {
				fRec39_perm[j47] = fRec39_tmp[vsize + j47];
			}
			/* Recursive loop 89 */
			/* Pre code */
			for (int j68 = 0; j68 < 4; j68 = j68 + 1) {
				fRec81_tmp[j68] = fRec81_perm[j68];
			}
			for (int j70 = 0; j70 < 4; j70 = j70 + 1) {
				fRec82_tmp[j70] = fRec82_perm[j70];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec108[i] = 0.62f * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec104[i], -0.6f), 0.35f) : fZec104[i] - 2.0f * fZec107[i] * std::max<float>(0.0f, fZec106[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec104[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec103[i] * (0.3128f - 0.010201851f * fZec105[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec106[i]))) * fZec107[i]))) : ((iSlow5) ? ((iSlow12) ? 0.3128f * (fZec103[i] / (fZec106[i] + 1.0f)) : ((iSlow13) ? 0.3128f * (fZec103[i] / std::sqrt(0.09784384f * fZec105[i] + 1.0f)) : 0.63661975f * std::atan(0.49134508f * fZec103[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec104[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec104[i], -0.5f), 0.5f) : tanhf(fZec104[i]))))) - (fConst4 * fRec81[i - 1] + fRec82[i - 1]);
				fRec81[i] = fRec81[i - 1] + fConst8 * fZec108[i];
				fZec109[i] = fRec81[i - 1] + fConst7 * fZec108[i];
				fRec82[i] = fRec82[i - 1] + fConst9 * fZec109[i];
				fZec110[i] = fConst3 * fZec109[i];
				fRec83[i] = fRec82[i - 1] + fZec110[i];
				fZec111[i] = fConst10 * fZec108[i];
				fRec84[i] = fZec111[i];
				fRec85[i] = fZec109[i];
			}
			/* Post code */
			for (int j69 = 0; j69 < 4; j69 = j69 + 1) {
				fRec81_perm[j69] = fRec81_tmp[vsize + j69];
			}
			for (int j71 = 0; j71 < 4; j71 = j71 + 1) {
				fRec82_perm[j71] = fRec82_tmp[vsize + j71];
			}
			/* Recursive loop 90 */
			/* Pre code */
			for (int j96 = 0; j96 < 4; j96 = j96 + 1) {
				fRec90_tmp[j96] = fRec90_perm[j96];
			}
			for (int j98 = 0; j98 < 4; j98 = j98 + 1) {
				fRec91_tmp[j98] = fRec91_perm[j98];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec151[i] = fSlow27 * (fRec96[i] + fRec97[i] * fZec35[i] + 1.4144272f * fRec98[i] * fZec36[i]) - (fConst26 * fRec90[i - 1] + fRec91[i - 1]);
				fRec90[i] = fRec90[i - 1] + fConst29 * fZec151[i];
				fZec152[i] = fRec90[i - 1] + fConst28 * fZec151[i];
				fRec91[i] = fRec91[i - 1] + fConst30 * fZec152[i];
				fRec92[i] = fZec152[i];
				fZec153[i] = fConst31 * fZec151[i];
				fZec154[i] = fConst25 * fZec152[i];
				fRec93[i] = fZec154[i] + fRec91[i - 1] + fZec153[i];
			}
			/* Post code */
			for (int j97 = 0; j97 < 4; j97 = j97 + 1) {
				fRec90_perm[j97] = fRec90_tmp[vsize + j97];
			}
			for (int j99 = 0; j99 < 4; j99 = j99 + 1) {
				fRec91_perm[j99] = fRec91_tmp[vsize + j99];
			}
			/* Recursive loop 91 */
			/* Pre code */
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec20_tmp[j16] = fRec20_perm[j16];
			}
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec21_tmp[j18] = fRec21_perm[j18];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec26[i] = fRec27[i] + fRec26[i] * fZec24[i] + 1.4144272f * fRec28[i] * fZec25[i] - (fConst12 * fRec20[i - 1] + fRec21[i - 1]);
				fRec20[i] = fRec20[i - 1] + fConst15 * fZec26[i];
				fZec27[i] = fRec20[i - 1] + fConst14 * fZec26[i];
				fRec21[i] = fRec21[i - 1] + fConst16 * fZec27[i];
				fRec22[i] = fZec27[i];
				fZec28[i] = fConst17 * fZec26[i];
				fZec29[i] = fConst11 * fZec27[i];
				fRec23[i] = fZec29[i] + fRec21[i - 1] + fZec28[i];
			}
			/* Post code */
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec20_perm[j17] = fRec20_tmp[vsize + j17];
			}
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec21_perm[j19] = fRec21_tmp[vsize + j19];
			}
			/* Vectorizable loop 92 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec72[i] = fRec41[i] + fSlow28 * fRec40[i];
			}
			/* Recursive loop 93 */
			/* Pre code */
			for (int j72 = 0; j72 < 4; j72 = j72 + 1) {
				fRec77_tmp[j72] = fRec77_perm[j72];
			}
			for (int j74 = 0; j74 < 4; j74 = j74 + 1) {
				fRec78_tmp[j74] = fRec78_perm[j74];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec112[i] = fRec84[i] + fRec83[i] * fZec24[i] + 1.4144272f * fRec85[i] * fZec25[i] - (fConst12 * fRec77[i - 1] + fRec78[i - 1]);
				fRec77[i] = fRec77[i - 1] + fConst15 * fZec112[i];
				fZec113[i] = fRec77[i - 1] + fConst14 * fZec112[i];
				fRec78[i] = fRec78[i - 1] + fConst16 * fZec113[i];
				fRec79[i] = fZec113[i];
				fZec114[i] = fConst17 * fZec112[i];
				fZec115[i] = fConst11 * fZec113[i];
				fRec80[i] = fZec115[i] + fRec78[i - 1] + fZec114[i];
			}
			/* Post code */
			for (int j73 = 0; j73 < 4; j73 = j73 + 1) {
				fRec77_perm[j73] = fRec77_tmp[vsize + j73];
			}
			for (int j75 = 0; j75 < 4; j75 = j75 + 1) {
				fRec78_perm[j75] = fRec78_tmp[vsize + j75];
			}
			/* Vectorizable loop 94 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec155[i] = fRec93[i] + fSlow28 * fRec92[i];
			}
			/* Recursive loop 95 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec15_tmp[j22] = fRec15_perm[j22];
			}
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec16_tmp[j24] = fRec16_perm[j24];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec31[i] = fRec23[i] + fRec22[i] * fZec30[i] - (fConst19 * fRec15[i - 1] + fRec16[i - 1]);
				fRec15[i] = fRec15[i - 1] + fConst22 * fZec31[i];
				fZec32[i] = fRec15[i - 1] + fConst21 * fZec31[i];
				fRec16[i] = fRec16[i - 1] + fConst23 * fZec32[i];
				fZec33[i] = fConst18 * fZec32[i];
				fRec17[i] = fRec16[i - 1] + fZec33[i];
				fZec34[i] = fConst24 * fZec31[i];
				fRec18[i] = fZec34[i];
				fRec19[i] = fZec32[i];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec15_perm[j23] = fRec15_tmp[vsize + j23];
			}
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec16_perm[j25] = fRec16_tmp[vsize + j25];
			}
			/* Vectorizable loop 96 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec73[i] = std::fabs(fZec72[i]);
			}
			/* Vectorizable loop 97 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec74[i] = ((fZec72[i] > 0.0f) ? 1.0f : ((fZec72[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 98 */
			/* Pre code */
			for (int j76 = 0; j76 < 4; j76 = j76 + 1) {
				fRec72_tmp[j76] = fRec72_perm[j76];
			}
			for (int j78 = 0; j78 < 4; j78 = j78 + 1) {
				fRec73_tmp[j78] = fRec73_perm[j78];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec116[i] = fRec80[i] + fRec79[i] * fZec30[i] - (fConst19 * fRec72[i - 1] + fRec73[i - 1]);
				fRec72[i] = fRec72[i - 1] + fConst22 * fZec116[i];
				fZec117[i] = fRec72[i - 1] + fConst21 * fZec116[i];
				fRec73[i] = fRec73[i - 1] + fConst23 * fZec117[i];
				fZec118[i] = fConst18 * fZec117[i];
				fRec74[i] = fRec73[i - 1] + fZec118[i];
				fZec119[i] = fConst24 * fZec116[i];
				fRec75[i] = fZec119[i];
				fRec76[i] = fZec117[i];
			}
			/* Post code */
			for (int j77 = 0; j77 < 4; j77 = j77 + 1) {
				fRec72_perm[j77] = fRec72_tmp[vsize + j77];
			}
			for (int j79 = 0; j79 < 4; j79 = j79 + 1) {
				fRec73_perm[j79] = fRec73_tmp[vsize + j79];
			}
			/* Vectorizable loop 99 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec156[i] = std::fabs(fZec155[i]);
			}
			/* Vectorizable loop 100 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec157[i] = ((fZec155[i] > 0.0f) ? 1.0f : ((fZec155[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 101 */
			/* Pre code */
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec11_tmp[j28] = fRec11_perm[j28];
			}
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec12_tmp[j30] = fRec12_perm[j30];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec37[i] = fSlow27 * (fRec17[i] + fRec18[i] * fZec35[i] + 1.4144272f * fRec19[i] * fZec36[i]) - (fConst26 * fRec11[i - 1] + fRec12[i - 1]);
				fRec11[i] = fRec11[i - 1] + fConst29 * fZec37[i];
				fZec38[i] = fRec11[i - 1] + fConst28 * fZec37[i];
				fRec12[i] = fRec12[i - 1] + fConst30 * fZec38[i];
				fRec13[i] = fZec38[i];
				fZec39[i] = fConst31 * fZec37[i];
				fZec40[i] = fConst25 * fZec38[i];
				fRec14[i] = fZec40[i] + fRec12[i - 1] + fZec39[i];
			}
			/* Post code */
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec11_perm[j29] = fRec11_tmp[vsize + j29];
			}
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec12_perm[j31] = fRec12_tmp[vsize + j31];
			}
			/* Vectorizable loop 102 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec75[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec72[i], -0.6f), 0.35f) : fZec72[i] - 2.0f * fZec74[i] * std::max<float>(0.0f, fZec73[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec72[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec72[i] - 0.33333334f * mlczerov_faustpower3_f(fZec72[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec73[i]))) * fZec74[i]))) : ((iSlow5) ? ((iSlow12) ? fZec72[i] / (fZec73[i] + 1.0f) : ((iSlow13) ? fZec72[i] / std::sqrt(mlczerov_faustpower2_f(fZec72[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec72[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec72[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec72[i], -0.5f), 0.5f) : tanhf(fZec72[i])))));
			}
			/* Recursive loop 103 */
			/* Pre code */
			for (int j80 = 0; j80 < 4; j80 = j80 + 1) {
				fRec68_tmp[j80] = fRec68_perm[j80];
			}
			for (int j82 = 0; j82 < 4; j82 = j82 + 1) {
				fRec69_tmp[j82] = fRec69_perm[j82];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec120[i] = fSlow27 * (fRec74[i] + fRec75[i] * fZec35[i] + 1.4144272f * fRec76[i] * fZec36[i]) - (fConst26 * fRec68[i - 1] + fRec69[i - 1]);
				fRec68[i] = fRec68[i - 1] + fConst29 * fZec120[i];
				fZec121[i] = fRec68[i - 1] + fConst28 * fZec120[i];
				fRec69[i] = fRec69[i - 1] + fConst30 * fZec121[i];
				fRec70[i] = fZec121[i];
				fZec122[i] = fConst31 * fZec120[i];
				fZec123[i] = fConst25 * fZec121[i];
				fRec71[i] = fZec123[i] + fRec69[i - 1] + fZec122[i];
			}
			/* Post code */
			for (int j81 = 0; j81 < 4; j81 = j81 + 1) {
				fRec68_perm[j81] = fRec68_tmp[vsize + j81];
			}
			for (int j83 = 0; j83 < 4; j83 = j83 + 1) {
				fRec69_perm[j83] = fRec69_tmp[vsize + j83];
			}
			/* Vectorizable loop 104 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec158[i] = ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec155[i], -0.6f), 0.35f) : fZec155[i] - 2.0f * fZec157[i] * std::max<float>(0.0f, fZec156[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec155[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec155[i] - 0.33333334f * mlczerov_faustpower3_f(fZec155[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec156[i]))) * fZec157[i]))) : ((iSlow5) ? ((iSlow12) ? fZec155[i] / (fZec156[i] + 1.0f) : ((iSlow13) ? fZec155[i] / std::sqrt(mlczerov_faustpower2_f(fZec155[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec155[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec155[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec155[i], -0.5f), 0.5f) : tanhf(fZec155[i])))));
			}
			/* Recursive loop 105 */
			/* Pre code */
			for (int j48 = 0; j48 < 4; j48 = j48 + 1) {
				fRec37_tmp[j48] = fRec37_perm[j48];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec37[i] = std::max<float>(0.995f * fRec37[i - 1], std::fabs(fSlow29 * fZec75[i]));
			}
			/* Post code */
			for (int j49 = 0; j49 < 4; j49 = j49 + 1) {
				fRec37_perm[j49] = fRec37_tmp[vsize + j49];
			}
			/* Vectorizable loop 106 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec76[i] = fRec14[i] + fSlow28 * fRec13[i];
			}
			/* Recursive loop 107 */
			/* Pre code */
			for (int j56 = 0; j56 < 4; j56 = j56 + 1) {
				fRec56_tmp[j56] = fRec56_perm[j56];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec56[i] = fSlow31 + fConst2 * fRec56[i - 1];
			}
			/* Post code */
			for (int j57 = 0; j57 < 4; j57 = j57 + 1) {
				fRec56_perm[j57] = fRec56_tmp[vsize + j57];
			}
			/* Recursive loop 108 */
			/* Pre code */
			for (int j100 = 0; j100 < 4; j100 = j100 + 1) {
				fRec89_tmp[j100] = fRec89_perm[j100];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec89[i] = std::max<float>(0.995f * fRec89[i - 1], std::fabs(fSlow29 * fZec158[i]));
			}
			/* Post code */
			for (int j101 = 0; j101 < 4; j101 = j101 + 1) {
				fRec89_perm[j101] = fRec89_tmp[vsize + j101];
			}
			/* Vectorizable loop 109 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec159[i] = fRec71[i] + fSlow28 * fRec70[i];
			}
			/* Recursive loop 110 */
			/* Pre code */
			for (int j50 = 0; j50 < 4; j50 = j50 + 1) {
				fRec36_tmp[j50] = fRec36_perm[j50];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec36[i] = fConst1 * static_cast<float>(fRec37[i] > fZec0[i]) + fConst2 * fRec36[i - 1];
			}
			/* Post code */
			for (int j51 = 0; j51 < 4; j51 = j51 + 1) {
				fRec36_perm[j51] = fRec36_tmp[vsize + j51];
			}
			/* Vectorizable loop 111 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec77[i] = std::fabs(fZec76[i]);
			}
			/* Vectorizable loop 112 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec78[i] = ((fZec76[i] > 0.0f) ? 1.0f : ((fZec76[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Vectorizable loop 113 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec83[i] = std::pow(1e+01f, fSlow33 * fRec56[i]);
			}
			/* Recursive loop 114 */
			/* Pre code */
			for (int j62 = 0; j62 < 4; j62 = j62 + 1) {
				fRec57_tmp[j62] = fRec57_perm[j62];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec57[i] = fSlow34 + fConst2 * fRec57[i - 1];
			}
			/* Post code */
			for (int j63 = 0; j63 < 4; j63 = j63 + 1) {
				fRec57_perm[j63] = fRec57_tmp[vsize + j63];
			}
			/* Recursive loop 115 */
			/* Pre code */
			for (int j102 = 0; j102 < 4; j102 = j102 + 1) {
				fRec88_tmp[j102] = fRec88_perm[j102];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec88[i] = fConst1 * static_cast<float>(fRec89[i] > fZec0[i]) + fConst2 * fRec88[i - 1];
			}
			/* Post code */
			for (int j103 = 0; j103 < 4; j103 = j103 + 1) {
				fRec88_perm[j103] = fRec88_tmp[vsize + j103];
			}
			/* Vectorizable loop 116 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec160[i] = std::fabs(fZec159[i]);
			}
			/* Vectorizable loop 117 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec161[i] = ((fZec159[i] > 0.0f) ? 1.0f : ((fZec159[i] < 0.0f) ? -1.0f : 0.0f));
			}
			/* Recursive loop 118 */
			/* Pre code */
			for (int j52 = 0; j52 < 4; j52 = j52 + 1) {
				fRec6_tmp[j52] = fRec6_perm[j52];
			}
			for (int j54 = 0; j54 < 4; j54 = j54 + 1) {
				fRec7_tmp[j54] = fRec7_perm[j54];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec79[i] = ((iSlow30) ? fSlow29 * fRec36[i] * fZec75[i] : fSlow29 * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec76[i], -0.6f), 0.35f) : fZec76[i] - 2.0f * fZec78[i] * std::max<float>(0.0f, fZec77[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec76[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec76[i] - 0.33333334f * mlczerov_faustpower3_f(fZec76[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec77[i]))) * fZec78[i]))) : ((iSlow5) ? ((iSlow12) ? fZec76[i] / (fZec77[i] + 1.0f) : ((iSlow13) ? fZec76[i] / std::sqrt(mlczerov_faustpower2_f(fZec76[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec76[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec76[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec76[i], -0.5f), 0.5f) : tanhf(fZec76[i])))))) - (fConst33 * fRec6[i - 1] + fRec7[i - 1]);
				fRec6[i] = fRec6[i - 1] + fConst36 * fZec79[i];
				fZec80[i] = fRec6[i - 1] + fConst35 * fZec79[i];
				fRec7[i] = fRec7[i - 1] + fConst37 * fZec80[i];
				fZec81[i] = fConst32 * fZec80[i];
				fRec8[i] = fRec7[i - 1] + fZec81[i];
				fZec82[i] = fConst38 * fZec79[i];
				fRec9[i] = fZec82[i];
				fRec10[i] = fZec80[i];
			}
			/* Post code */
			for (int j53 = 0; j53 < 4; j53 = j53 + 1) {
				fRec6_perm[j53] = fRec6_tmp[vsize + j53];
			}
			for (int j55 = 0; j55 < 4; j55 = j55 + 1) {
				fRec7_perm[j55] = fRec7_tmp[vsize + j55];
			}
			/* Vectorizable loop 119 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec84[i] = std::sqrt(fZec83[i]);
			}
			/* Vectorizable loop 120 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec89[i] = std::pow(1e+01f, fSlow35 * fRec57[i]);
			}
			/* Recursive loop 121 */
			/* Pre code */
			for (int j104 = 0; j104 < 4; j104 = j104 + 1) {
				fRec63_tmp[j104] = fRec63_perm[j104];
			}
			for (int j106 = 0; j106 < 4; j106 = j106 + 1) {
				fRec64_tmp[j106] = fRec64_perm[j106];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec162[i] = ((iSlow30) ? fSlow29 * fRec88[i] * fZec158[i] : fSlow29 * ((iSlow4) ? ((iSlow17) ? ((iSlow21) ? std::min<float>(std::max<float>(fZec159[i], -0.6f), 0.35f) : fZec159[i] - 2.0f * fZec161[i] * std::max<float>(0.0f, fZec160[i] + -0.6f)) : ((iSlow18) ? tanhf(fZec159[i] + 0.25f) - fConst5 : ((iSlow19) ? std::min<float>(std::max<float>(fZec159[i] - 0.33333334f * mlczerov_faustpower3_f(fZec159[i]), -0.667f), 0.667f) : (1.0f - std::exp(-(fZec160[i]))) * fZec161[i]))) : ((iSlow5) ? ((iSlow12) ? fZec159[i] / (fZec160[i] + 1.0f) : ((iSlow13) ? fZec159[i] / std::sqrt(mlczerov_faustpower2_f(fZec159[i]) + 1.0f) : 0.63661975f * std::atan(1.5707964f * fZec159[i]))) : ((iSlow6) ? std::sin(std::max<float>(-1.5707964f, std::min<float>(1.5707964f, fZec159[i]))) : ((iSlow7) ? std::min<float>(std::max<float>(fZec159[i], -0.5f), 0.5f) : tanhf(fZec159[i])))))) - (fConst33 * fRec63[i - 1] + fRec64[i - 1]);
				fRec63[i] = fRec63[i - 1] + fConst36 * fZec162[i];
				fZec163[i] = fRec63[i - 1] + fConst35 * fZec162[i];
				fRec64[i] = fRec64[i - 1] + fConst37 * fZec163[i];
				fZec164[i] = fConst32 * fZec163[i];
				fRec65[i] = fRec64[i - 1] + fZec164[i];
				fZec165[i] = fConst38 * fZec162[i];
				fRec66[i] = fZec165[i];
				fRec67[i] = fZec163[i];
			}
			/* Post code */
			for (int j105 = 0; j105 < 4; j105 = j105 + 1) {
				fRec63_perm[j105] = fRec63_tmp[vsize + j105];
			}
			for (int j107 = 0; j107 < 4; j107 = j107 + 1) {
				fRec64_perm[j107] = fRec64_tmp[vsize + j107];
			}
			/* Recursive loop 122 */
			/* Pre code */
			for (int j58 = 0; j58 < 4; j58 = j58 + 1) {
				fRec1_tmp[j58] = fRec1_perm[j58];
			}
			for (int j60 = 0; j60 < 4; j60 = j60 + 1) {
				fRec2_tmp[j60] = fRec2_perm[j60];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec85[i] = fRec9[i] + fRec8[i] * fZec83[i] + 1.25f * fRec10[i] * fZec84[i] - (fConst40 * fRec1[i - 1] + fRec2[i - 1]);
				fRec1[i] = fRec1[i - 1] + fConst43 * fZec85[i];
				fZec86[i] = fRec1[i - 1] + fConst42 * fZec85[i];
				fRec2[i] = fRec2[i - 1] + fConst44 * fZec86[i];
				fZec87[i] = fConst39 * fZec86[i];
				fRec3[i] = fRec2[i - 1] + fZec87[i];
				fZec88[i] = fConst45 * fZec85[i];
				fRec4[i] = fZec88[i];
				fRec5[i] = fZec86[i];
			}
			/* Post code */
			for (int j59 = 0; j59 < 4; j59 = j59 + 1) {
				fRec1_perm[j59] = fRec1_tmp[vsize + j59];
			}
			for (int j61 = 0; j61 < 4; j61 = j61 + 1) {
				fRec2_perm[j61] = fRec2_tmp[vsize + j61];
			}
			/* Recursive loop 123 */
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
			/* Vectorizable loop 124 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec90[i] = std::sqrt(fZec89[i]);
			}
			/* Recursive loop 125 */
			/* Pre code */
			for (int j108 = 0; j108 < 4; j108 = j108 + 1) {
				fRec58_tmp[j108] = fRec58_perm[j108];
			}
			for (int j110 = 0; j110 < 4; j110 = j110 + 1) {
				fRec59_tmp[j110] = fRec59_perm[j110];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec166[i] = fRec66[i] + fRec65[i] * fZec83[i] + 1.25f * fRec67[i] * fZec84[i] - (fConst40 * fRec58[i - 1] + fRec59[i - 1]);
				fRec58[i] = fRec58[i - 1] + fConst43 * fZec166[i];
				fZec167[i] = fRec58[i - 1] + fConst42 * fZec166[i];
				fRec59[i] = fRec59[i - 1] + fConst44 * fZec167[i];
				fZec168[i] = fConst39 * fZec167[i];
				fRec60[i] = fRec59[i - 1] + fZec168[i];
				fZec169[i] = fConst45 * fZec166[i];
				fRec61[i] = fZec169[i];
				fRec62[i] = fZec167[i];
			}
			/* Post code */
			for (int j109 = 0; j109 < 4; j109 = j109 + 1) {
				fRec58_perm[j109] = fRec58_tmp[vsize + j109];
			}
			for (int j111 = 0; j111 < 4; j111 = j111 + 1) {
				fRec59_perm[j111] = fRec59_tmp[vsize + j111];
			}
			/* Vectorizable loop 126 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec3[i] + fRec4[i] * fZec89[i] + 1.4285715f * fRec5[i] * fZec90[i]));
			}
			/* Vectorizable loop 127 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fRec0[i] * (fRec60[i] + fRec61[i] * fZec89[i] + 1.4285715f * fRec62[i] * fZec90[i]));
			}
		}
	}

};

#endif
