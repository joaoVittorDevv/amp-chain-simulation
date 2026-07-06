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
	FAUSTFLOAT fHslider5;
	float fRec33[2];
	FAUSTFLOAT fHslider6;
	float fRec35[2];
	float fRec36[2];
	float fRec34[2];
	FAUSTFLOAT fEntry2;
	FAUSTFLOAT fEntry3;
	float fConst15;
	float fConst16;
	float fConst17;
	float fRec28[2];
	float fConst18;
	float fRec29[2];
	float fConst19;
	float fConst20;
	float fConst21;
	float fConst22;
	float fRec23[2];
	float fConst23;
	float fRec24[2];
	float fConst24;
	float fConst25;
	float fConst26;
	float fConst27;
	float fRec17[2];
	float fConst28;
	float fRec18[2];
	float fConst29;
	FAUSTFLOAT fEntry4;
	float fConst30;
	float fConst31;
	float fConst32;
	float fRec12[2];
	float fConst33;
	float fRec13[2];
	float fConst34;
	float fRec50[2];
	float fRec51[2];
	float fRec46[2];
	float fRec47[2];
	float fRec41[2];
	float fRec42[2];
	float fRec37[2];
	float fRec38[2];
	float fRec56[2];
	float fRec55[2];
	float fConst35;
	float fConst36;
	float fConst37;
	float fRec7[2];
	float fConst38;
	float fRec8[2];
	float fConst39;
	float fConst40;
	float fConst41;
	float fConst42;
	float fRec1[2];
	float fConst43;
	float fRec2[2];
	float fConst44;
	FAUSTFLOAT fHslider7;
	float fRec57[2];
	float fRec87[2];
	float fRec86[2];
	float fRec81[2];
	float fRec82[2];
	float fRec77[2];
	float fRec78[2];
	float fRec72[2];
	float fRec73[2];
	float fRec68[2];
	float fRec69[2];
	float fRec101[2];
	float fRec102[2];
	float fRec97[2];
	float fRec98[2];
	float fRec92[2];
	float fRec93[2];
	float fRec88[2];
	float fRec89[2];
	float fRec107[2];
	float fRec106[2];
	float fRec63[2];
	float fRec64[2];
	float fRec58[2];
	float fRec59[2];
	
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
		fConst15 = fConst13 * fConst14 + 1.0f;
		fConst16 = fConst13 / fConst15;
		fConst17 = 2.0f * fConst16;
		fConst18 = 2.0f * fConst13;
		fConst19 = 1.0f / fConst15;
		fConst20 = fConst11 * fConst12 + 1.0f;
		fConst21 = fConst11 / fConst20;
		fConst22 = 2.0f * fConst21;
		fConst23 = 2.0f * fConst11;
		fConst24 = 1.0f / fConst20;
		fConst25 = fConst9 * fConst10 + 1.0f;
		fConst26 = fConst9 / fConst25;
		fConst27 = 2.0f * fConst26;
		fConst28 = 2.0f * fConst9;
		fConst29 = 1.0f / fConst25;
		fConst30 = fConst7 * fConst8 + 1.0f;
		fConst31 = fConst7 / fConst30;
		fConst32 = 2.0f * fConst31;
		fConst33 = 2.0f * fConst7;
		fConst34 = 1.0f / fConst30;
		fConst35 = fConst5 * fConst6 + 1.0f;
		fConst36 = fConst5 / fConst35;
		fConst37 = 2.0f * fConst36;
		fConst38 = 2.0f * fConst5;
		fConst39 = 1.0f / fConst35;
		fConst40 = fConst3 * fConst4 + 1.0f;
		fConst41 = fConst3 / fConst40;
		fConst42 = 2.0f * fConst41;
		fConst43 = 2.0f * fConst3;
		fConst44 = 1.0f / fConst40;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry0 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider1 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry1 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider2 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider5 = static_cast<FAUSTFLOAT>(0.25118864f);
		fHslider6 = static_cast<FAUSTFLOAT>(-8e+01f);
		fEntry2 = static_cast<FAUSTFLOAT>(0.0f);
		fEntry3 = static_cast<FAUSTFLOAT>(1.0f);
		fEntry4 = static_cast<FAUSTFLOAT>(0.0f);
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
			fRec35[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 2; l7 = l7 + 1) {
			fRec36[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 2; l8 = l8 + 1) {
			fRec34[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 2; l9 = l9 + 1) {
			fRec28[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 2; l10 = l10 + 1) {
			fRec29[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 2; l11 = l11 + 1) {
			fRec23[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 2; l12 = l12 + 1) {
			fRec24[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 2; l13 = l13 + 1) {
			fRec17[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 2; l14 = l14 + 1) {
			fRec18[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 2; l15 = l15 + 1) {
			fRec12[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 2; l16 = l16 + 1) {
			fRec13[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 2; l17 = l17 + 1) {
			fRec50[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 2; l18 = l18 + 1) {
			fRec51[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 2; l19 = l19 + 1) {
			fRec46[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 2; l20 = l20 + 1) {
			fRec47[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 2; l21 = l21 + 1) {
			fRec41[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 2; l22 = l22 + 1) {
			fRec42[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 2; l23 = l23 + 1) {
			fRec37[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 2; l24 = l24 + 1) {
			fRec38[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 2; l25 = l25 + 1) {
			fRec56[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 2; l26 = l26 + 1) {
			fRec55[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 2; l27 = l27 + 1) {
			fRec7[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 2; l28 = l28 + 1) {
			fRec8[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 2; l29 = l29 + 1) {
			fRec1[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 2; l30 = l30 + 1) {
			fRec2[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 2; l31 = l31 + 1) {
			fRec57[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 2; l32 = l32 + 1) {
			fRec87[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 2; l33 = l33 + 1) {
			fRec86[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 2; l34 = l34 + 1) {
			fRec81[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 2; l35 = l35 + 1) {
			fRec82[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 2; l36 = l36 + 1) {
			fRec77[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 2; l37 = l37 + 1) {
			fRec78[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 2; l38 = l38 + 1) {
			fRec72[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 2; l39 = l39 + 1) {
			fRec73[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 2; l40 = l40 + 1) {
			fRec68[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 2; l41 = l41 + 1) {
			fRec69[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 2; l42 = l42 + 1) {
			fRec101[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 2; l43 = l43 + 1) {
			fRec102[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 2; l44 = l44 + 1) {
			fRec97[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 2; l45 = l45 + 1) {
			fRec98[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 2; l46 = l46 + 1) {
			fRec92[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 2; l47 = l47 + 1) {
			fRec93[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 2; l48 = l48 + 1) {
			fRec88[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 2; l49 = l49 + 1) {
			fRec89[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 2; l50 = l50 + 1) {
			fRec107[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 2; l51 = l51 + 1) {
			fRec106[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 2; l52 = l52 + 1) {
			fRec63[l52] = 0.0f;
		}
		for (int l53 = 0; l53 < 2; l53 = l53 + 1) {
			fRec64[l53] = 0.0f;
		}
		for (int l54 = 0; l54 < 2; l54 = l54 + 1) {
			fRec58[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 2; l55 = l55 + 1) {
			fRec59[l55] = 0.0f;
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
		ui_interface->declare(&fHslider4, "unit", "dB");
		ui_interface->addHorizontalSlider("Bass", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Bright", &fEntry3, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider1, "unit", "dB");
		ui_interface->addHorizontalSlider("Depth", &fHslider1, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("Feedback", &fEntry0, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Gain", &fHslider5, FAUSTFLOAT(0.25118864f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->addNumEntry("Gate Pos", &fEntry1, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider6, "unit", "dB");
		ui_interface->addHorizontalSlider("Gate", &fHslider6, FAUSTFLOAT(-8e+01f), FAUSTFLOAT(-8e+01f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("M45", &fEntry2, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("Master", &fHslider7, FAUSTFLOAT(0.5011872f), FAUSTFLOAT(0.001f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0001f));
		ui_interface->declare(&fHslider3, "unit", "dB");
		ui_interface->addHorizontalSlider("Middle", &fHslider3, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider0, "unit", "dB");
		ui_interface->addHorizontalSlider("Presence", &fHslider0, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider2, "unit", "dB");
		ui_interface->addHorizontalSlider("Treble", &fHslider2, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addNumEntry("WARCLAW", &fEntry4, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(1.0f));
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
		float fSlow6 = fConst1 * static_cast<float>(fHslider2);
		float fSlow7 = fConst1 * static_cast<float>(fHslider3);
		float fSlow8 = fConst1 * static_cast<float>(fHslider4);
		float fSlow9 = fConst1 * static_cast<float>(fHslider5);
		float fSlow10 = fConst1 * static_cast<float>(fHslider6);
		float fSlow11 = 1.0f - 0.35f * static_cast<float>(fEntry2);
		float fSlow12 = 0.22f * (1.2f * static_cast<float>(fEntry3) + 1.5f) * fSlow11;
		float fSlow13 = 0.2652f * fSlow11;
		float fSlow14 = static_cast<float>(fEntry4);
		float fSlow15 = 1.9f * fSlow14 + 1.0f;
		float fSlow16 = std::pow(1e+01f, 0.2f * fSlow14);
		float fSlow17 = 1.0f - 0.22f * fSlow14;
		float fSlow18 = fConst1 * static_cast<float>(fHslider7);
		for (int i0 = 0; i0 < count; i0 = i0 + 1) {
			fRec0[0] = fSlow0 + fConst2 * fRec0[1];
			float fTemp0 = std::pow(1e+01f, fSlow2 * fRec0[0]);
			float fTemp1 = std::sqrt(fTemp0);
			fRec6[0] = fSlow3 + fConst2 * fRec6[1];
			float fTemp2 = std::pow(1e+01f, fSlow4 * fRec6[0]);
			float fTemp3 = std::sqrt(fTemp2);
			fRec16[0] = fSlow6 + fConst2 * fRec16[1];
			float fTemp4 = std::pow(1e+01f, 0.05f * fRec16[0]);
			float fTemp5 = std::sqrt(fTemp4);
			fRec22[0] = fSlow7 + fConst2 * fRec22[1];
			float fTemp6 = std::pow(1e+01f, 0.05f * (fRec22[0] + -2.5f));
			fRec27[0] = fSlow8 + fConst2 * fRec27[1];
			float fTemp7 = std::pow(1e+01f, 0.05f * fRec27[0]);
			float fTemp8 = std::sqrt(fTemp7);
			fRec33[0] = fSlow9 + fConst2 * fRec33[1];
			float fTemp9 = 72.0f * fRec33[0] + 8.0f;
			fRec35[0] = fSlow10 + fConst2 * fRec35[1];
			float fTemp10 = std::pow(1e+01f, 0.05f * fRec35[0]);
			float fTemp11 = static_cast<float>(input0[i0]);
			fRec36[0] = std::max<float>(0.995f * fRec36[1], std::fabs(fTemp11));
			fRec34[0] = fConst1 * static_cast<float>(fRec36[0] > fTemp10) + fConst2 * fRec34[1];
			float fTemp12 = 0.62f * tanhf(0.3128f * fTemp9 * tanhf(fSlow13 * fTemp9 * tanhf(fSlow12 * fTemp11 * fRec34[0] * fRec33[0] * fTemp9) + 0.03f)) - (fConst14 * fRec28[1] + fRec29[1]);
			fRec28[0] = fRec28[1] + fConst17 * fTemp12;
			float fTemp13 = fRec28[1] + fConst16 * fTemp12;
			fRec29[0] = fRec29[1] + fConst18 * fTemp13;
			float fTemp14 = fConst13 * fTemp13;
			float fRec30 = fRec29[1] + fTemp14;
			float fTemp15 = fConst19 * fTemp12;
			float fRec31 = fTemp15;
			float fRec32 = fTemp13;
			float fTemp16 = fRec31 + fRec30 * fTemp7 + 1.4144272f * fRec32 * fTemp8 - (fConst12 * fRec23[1] + fRec24[1]);
			fRec23[0] = fRec23[1] + fConst22 * fTemp16;
			float fTemp17 = fRec23[1] + fConst21 * fTemp16;
			fRec24[0] = fRec24[1] + fConst23 * fTemp17;
			float fRec25 = fTemp17;
			float fTemp18 = fConst24 * fTemp16;
			float fTemp19 = fConst11 * fTemp17;
			float fRec26 = fTemp19 + fRec24[1] + fTemp18;
			float fTemp20 = fRec26 + fRec25 * fTemp6 - (fConst10 * fRec17[1] + fRec18[1]);
			fRec17[0] = fRec17[1] + fConst27 * fTemp20;
			float fTemp21 = fRec17[1] + fConst26 * fTemp20;
			fRec18[0] = fRec18[1] + fConst28 * fTemp21;
			float fTemp22 = fConst9 * fTemp21;
			float fRec19 = fRec18[1] + fTemp22;
			float fTemp23 = fConst29 * fTemp20;
			float fRec20 = fTemp23;
			float fRec21 = fTemp21;
			float fTemp24 = fSlow15 * (fRec19 + fRec20 * fTemp4 + 1.4144272f * fRec21 * fTemp5) - (fConst8 * fRec12[1] + fRec13[1]);
			fRec12[0] = fRec12[1] + fConst32 * fTemp24;
			float fTemp25 = fRec12[1] + fConst31 * fTemp24;
			fRec13[0] = fRec13[1] + fConst33 * fTemp25;
			float fRec14 = fTemp25;
			float fTemp26 = fConst34 * fTemp24;
			float fTemp27 = fConst7 * fTemp25;
			float fRec15 = fTemp27 + fRec13[1] + fTemp26;
			float fTemp28 = 0.62f * tanhf(0.3128f * fTemp9 * tanhf(fSlow13 * fTemp9 * tanhf(fSlow12 * fTemp11 * fRec33[0] * fTemp9) + 0.03f)) - (fConst14 * fRec50[1] + fRec51[1]);
			fRec50[0] = fRec50[1] + fConst17 * fTemp28;
			float fTemp29 = fRec50[1] + fConst16 * fTemp28;
			fRec51[0] = fRec51[1] + fConst18 * fTemp29;
			float fTemp30 = fConst13 * fTemp29;
			float fRec52 = fRec51[1] + fTemp30;
			float fTemp31 = fConst19 * fTemp28;
			float fRec53 = fTemp31;
			float fRec54 = fTemp29;
			float fTemp32 = fRec53 + fRec52 * fTemp7 + 1.4144272f * fRec54 * fTemp8 - (fConst12 * fRec46[1] + fRec47[1]);
			fRec46[0] = fRec46[1] + fConst22 * fTemp32;
			float fTemp33 = fRec46[1] + fConst21 * fTemp32;
			fRec47[0] = fRec47[1] + fConst23 * fTemp33;
			float fRec48 = fTemp33;
			float fTemp34 = fConst24 * fTemp32;
			float fTemp35 = fConst11 * fTemp33;
			float fRec49 = fTemp35 + fRec47[1] + fTemp34;
			float fTemp36 = fRec49 + fRec48 * fTemp6 - (fConst10 * fRec41[1] + fRec42[1]);
			fRec41[0] = fRec41[1] + fConst27 * fTemp36;
			float fTemp37 = fRec41[1] + fConst26 * fTemp36;
			fRec42[0] = fRec42[1] + fConst28 * fTemp37;
			float fTemp38 = fConst9 * fTemp37;
			float fRec43 = fRec42[1] + fTemp38;
			float fTemp39 = fConst29 * fTemp36;
			float fRec44 = fTemp39;
			float fRec45 = fTemp37;
			float fTemp40 = fSlow15 * (fRec43 + fRec44 * fTemp4 + 1.4144272f * fRec45 * fTemp5) - (fConst8 * fRec37[1] + fRec38[1]);
			fRec37[0] = fRec37[1] + fConst32 * fTemp40;
			float fTemp41 = fRec37[1] + fConst31 * fTemp40;
			fRec38[0] = fRec38[1] + fConst33 * fTemp41;
			float fRec39 = fTemp41;
			float fTemp42 = fConst34 * fTemp40;
			float fTemp43 = fConst7 * fTemp41;
			float fRec40 = fTemp43 + fRec38[1] + fTemp42;
			float fTemp44 = tanhf(fRec40 + fSlow16 * fRec39);
			fRec56[0] = std::max<float>(0.995f * fRec56[1], std::fabs(fSlow17 * fTemp44));
			fRec55[0] = fConst1 * static_cast<float>(fRec56[0] > fTemp10) + fConst2 * fRec55[1];
			float fTemp45 = ((iSlow5) ? fSlow17 * fRec55[0] * fTemp44 : fSlow17 * tanhf(fRec15 + fSlow16 * fRec14)) - (fConst6 * fRec7[1] + fRec8[1]);
			fRec7[0] = fRec7[1] + fConst37 * fTemp45;
			float fTemp46 = fRec7[1] + fConst36 * fTemp45;
			fRec8[0] = fRec8[1] + fConst38 * fTemp46;
			float fTemp47 = fConst5 * fTemp46;
			float fRec9 = fRec8[1] + fTemp47;
			float fTemp48 = fConst39 * fTemp45;
			float fRec10 = fTemp48;
			float fRec11 = fTemp46;
			float fTemp49 = fRec10 + fRec9 * fTemp2 + 1.25f * fRec11 * fTemp3 - (fConst4 * fRec1[1] + fRec2[1]);
			fRec1[0] = fRec1[1] + fConst42 * fTemp49;
			float fTemp50 = fRec1[1] + fConst41 * fTemp49;
			fRec2[0] = fRec2[1] + fConst43 * fTemp50;
			float fTemp51 = fConst3 * fTemp50;
			float fRec3 = fRec2[1] + fTemp51;
			float fTemp52 = fConst44 * fTemp49;
			float fRec4 = fTemp52;
			float fRec5 = fTemp50;
			fRec57[0] = fSlow18 + fConst2 * fRec57[1];
			output0[i0] = static_cast<FAUSTFLOAT>(fRec57[0] * (fRec3 + fRec4 * fTemp0 + 1.4285715f * fRec5 * fTemp1));
			float fTemp53 = static_cast<float>(input1[i0]);
			fRec87[0] = std::max<float>(0.995f * fRec87[1], std::fabs(fTemp53));
			fRec86[0] = fConst1 * static_cast<float>(fRec87[0] > fTemp10) + fConst2 * fRec86[1];
			float fTemp54 = fTemp53 * fRec33[0];
			float fTemp55 = 0.62f * tanhf(0.3128f * fTemp9 * tanhf(fSlow13 * fTemp9 * tanhf(fSlow12 * fTemp54 * fRec86[0] * fTemp9) + 0.03f)) - (fConst14 * fRec81[1] + fRec82[1]);
			fRec81[0] = fRec81[1] + fConst17 * fTemp55;
			float fTemp56 = fRec81[1] + fConst16 * fTemp55;
			fRec82[0] = fRec82[1] + fConst18 * fTemp56;
			float fTemp57 = fConst13 * fTemp56;
			float fRec83 = fRec82[1] + fTemp57;
			float fTemp58 = fConst19 * fTemp55;
			float fRec84 = fTemp58;
			float fRec85 = fTemp56;
			float fTemp59 = fRec84 + fRec83 * fTemp7 + 1.4144272f * fRec85 * fTemp8 - (fConst12 * fRec77[1] + fRec78[1]);
			fRec77[0] = fRec77[1] + fConst22 * fTemp59;
			float fTemp60 = fRec77[1] + fConst21 * fTemp59;
			fRec78[0] = fRec78[1] + fConst23 * fTemp60;
			float fRec79 = fTemp60;
			float fTemp61 = fConst24 * fTemp59;
			float fTemp62 = fConst11 * fTemp60;
			float fRec80 = fTemp62 + fRec78[1] + fTemp61;
			float fTemp63 = fRec80 + fRec79 * fTemp6 - (fConst10 * fRec72[1] + fRec73[1]);
			fRec72[0] = fRec72[1] + fConst27 * fTemp63;
			float fTemp64 = fRec72[1] + fConst26 * fTemp63;
			fRec73[0] = fRec73[1] + fConst28 * fTemp64;
			float fTemp65 = fConst9 * fTemp64;
			float fRec74 = fRec73[1] + fTemp65;
			float fTemp66 = fConst29 * fTemp63;
			float fRec75 = fTemp66;
			float fRec76 = fTemp64;
			float fTemp67 = fSlow15 * (fRec74 + fRec75 * fTemp4 + 1.4144272f * fRec76 * fTemp5) - (fConst8 * fRec68[1] + fRec69[1]);
			fRec68[0] = fRec68[1] + fConst32 * fTemp67;
			float fTemp68 = fRec68[1] + fConst31 * fTemp67;
			fRec69[0] = fRec69[1] + fConst33 * fTemp68;
			float fRec70 = fTemp68;
			float fTemp69 = fConst34 * fTemp67;
			float fTemp70 = fConst7 * fTemp68;
			float fRec71 = fTemp70 + fRec69[1] + fTemp69;
			float fTemp71 = 0.62f * tanhf(0.3128f * fTemp9 * tanhf(fSlow13 * fTemp9 * tanhf(fSlow12 * fTemp54 * fTemp9) + 0.03f)) - (fConst14 * fRec101[1] + fRec102[1]);
			fRec101[0] = fRec101[1] + fConst17 * fTemp71;
			float fTemp72 = fRec101[1] + fConst16 * fTemp71;
			fRec102[0] = fRec102[1] + fConst18 * fTemp72;
			float fTemp73 = fConst13 * fTemp72;
			float fRec103 = fRec102[1] + fTemp73;
			float fTemp74 = fConst19 * fTemp71;
			float fRec104 = fTemp74;
			float fRec105 = fTemp72;
			float fTemp75 = fRec104 + fRec103 * fTemp7 + 1.4144272f * fRec105 * fTemp8 - (fConst12 * fRec97[1] + fRec98[1]);
			fRec97[0] = fRec97[1] + fConst22 * fTemp75;
			float fTemp76 = fRec97[1] + fConst21 * fTemp75;
			fRec98[0] = fRec98[1] + fConst23 * fTemp76;
			float fRec99 = fTemp76;
			float fTemp77 = fConst24 * fTemp75;
			float fTemp78 = fConst11 * fTemp76;
			float fRec100 = fTemp78 + fRec98[1] + fTemp77;
			float fTemp79 = fRec100 + fRec99 * fTemp6 - (fConst10 * fRec92[1] + fRec93[1]);
			fRec92[0] = fRec92[1] + fConst27 * fTemp79;
			float fTemp80 = fRec92[1] + fConst26 * fTemp79;
			fRec93[0] = fRec93[1] + fConst28 * fTemp80;
			float fTemp81 = fConst9 * fTemp80;
			float fRec94 = fRec93[1] + fTemp81;
			float fTemp82 = fConst29 * fTemp79;
			float fRec95 = fTemp82;
			float fRec96 = fTemp80;
			float fTemp83 = fSlow15 * (fRec94 + fRec95 * fTemp4 + 1.4144272f * fRec96 * fTemp5) - (fConst8 * fRec88[1] + fRec89[1]);
			fRec88[0] = fRec88[1] + fConst32 * fTemp83;
			float fTemp84 = fRec88[1] + fConst31 * fTemp83;
			fRec89[0] = fRec89[1] + fConst33 * fTemp84;
			float fRec90 = fTemp84;
			float fTemp85 = fConst34 * fTemp83;
			float fTemp86 = fConst7 * fTemp84;
			float fRec91 = fTemp86 + fRec89[1] + fTemp85;
			float fTemp87 = tanhf(fRec91 + fSlow16 * fRec90);
			fRec107[0] = std::max<float>(0.995f * fRec107[1], std::fabs(fSlow17 * fTemp87));
			fRec106[0] = fConst1 * static_cast<float>(fRec107[0] > fTemp10) + fConst2 * fRec106[1];
			float fTemp88 = ((iSlow5) ? fSlow17 * fRec106[0] * fTemp87 : fSlow17 * tanhf(fRec71 + fSlow16 * fRec70)) - (fConst6 * fRec63[1] + fRec64[1]);
			fRec63[0] = fRec63[1] + fConst37 * fTemp88;
			float fTemp89 = fRec63[1] + fConst36 * fTemp88;
			fRec64[0] = fRec64[1] + fConst38 * fTemp89;
			float fTemp90 = fConst5 * fTemp89;
			float fRec65 = fRec64[1] + fTemp90;
			float fTemp91 = fConst39 * fTemp88;
			float fRec66 = fTemp91;
			float fRec67 = fTemp89;
			float fTemp92 = fRec66 + fRec65 * fTemp2 + 1.25f * fRec67 * fTemp3 - (fConst4 * fRec58[1] + fRec59[1]);
			fRec58[0] = fRec58[1] + fConst42 * fTemp92;
			float fTemp93 = fRec58[1] + fConst41 * fTemp92;
			fRec59[0] = fRec59[1] + fConst43 * fTemp93;
			float fTemp94 = fConst3 * fTemp93;
			float fRec60 = fRec59[1] + fTemp94;
			float fTemp95 = fConst44 * fTemp92;
			float fRec61 = fTemp95;
			float fRec62 = fTemp93;
			output1[i0] = static_cast<FAUSTFLOAT>(fRec57[0] * (fRec60 + fRec61 * fTemp0 + 1.4285715f * fRec62 * fTemp1));
			fRec0[1] = fRec0[0];
			fRec6[1] = fRec6[0];
			fRec16[1] = fRec16[0];
			fRec22[1] = fRec22[0];
			fRec27[1] = fRec27[0];
			fRec33[1] = fRec33[0];
			fRec35[1] = fRec35[0];
			fRec36[1] = fRec36[0];
			fRec34[1] = fRec34[0];
			fRec28[1] = fRec28[0];
			fRec29[1] = fRec29[0];
			fRec23[1] = fRec23[0];
			fRec24[1] = fRec24[0];
			fRec17[1] = fRec17[0];
			fRec18[1] = fRec18[0];
			fRec12[1] = fRec12[0];
			fRec13[1] = fRec13[0];
			fRec50[1] = fRec50[0];
			fRec51[1] = fRec51[0];
			fRec46[1] = fRec46[0];
			fRec47[1] = fRec47[0];
			fRec41[1] = fRec41[0];
			fRec42[1] = fRec42[0];
			fRec37[1] = fRec37[0];
			fRec38[1] = fRec38[0];
			fRec56[1] = fRec56[0];
			fRec55[1] = fRec55[0];
			fRec7[1] = fRec7[0];
			fRec8[1] = fRec8[0];
			fRec1[1] = fRec1[0];
			fRec2[1] = fRec2[0];
			fRec57[1] = fRec57[0];
			fRec87[1] = fRec87[0];
			fRec86[1] = fRec86[0];
			fRec81[1] = fRec81[0];
			fRec82[1] = fRec82[0];
			fRec77[1] = fRec77[0];
			fRec78[1] = fRec78[0];
			fRec72[1] = fRec72[0];
			fRec73[1] = fRec73[0];
			fRec68[1] = fRec68[0];
			fRec69[1] = fRec69[0];
			fRec101[1] = fRec101[0];
			fRec102[1] = fRec102[0];
			fRec97[1] = fRec97[0];
			fRec98[1] = fRec98[0];
			fRec92[1] = fRec92[0];
			fRec93[1] = fRec93[0];
			fRec88[1] = fRec88[0];
			fRec89[1] = fRec89[0];
			fRec107[1] = fRec107[0];
			fRec106[1] = fRec106[0];
			fRec63[1] = fRec63[0];
			fRec64[1] = fRec64[0];
			fRec58[1] = fRec58[0];
			fRec59[1] = fRec59[0];
		}
	}

};

#endif
