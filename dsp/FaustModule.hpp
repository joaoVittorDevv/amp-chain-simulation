/* ------------------------------------------------------------
name: "main"
Code generated with Faust 2.85.1 (https://faust.grame.fr)
Compilation options: -lang cpp -i -fpga-mem-th 4 -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

#ifndef  __mydsp_H__
#define  __mydsp_H__

#ifndef FAUSTFLOAT
#define FAUSTFLOAT float
#endif 

/* link with : "" */
#include <algorithm>
#include <cmath>
#include <cstdint>
#include <math.h>

#ifndef FAUSTCLASS 
#define FAUSTCLASS mydsp
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


class mydsp : public dsp {
	
 private:
	
	int fSampleRate;
	float fConst0;
	float fConst1;
	float fConst2;
	FAUSTFLOAT fHslider0;
	float fRec0[2];
	FAUSTFLOAT fHslider1;
	float fRec1[2];
	FAUSTFLOAT fHslider2;
	float fRec7[2];
	float fConst3;
	FAUSTFLOAT fHslider3;
	float fRec8[2];
	FAUSTFLOAT fHslider4;
	float fRec13[2];
	FAUSTFLOAT fHslider5;
	float fRec14[2];
	FAUSTFLOAT fHslider6;
	float fRec15[2];
	FAUSTFLOAT fHslider7;
	float fRec16[2];
	FAUSTFLOAT fHslider8;
	float fRec22[2];
	float fRec17[2];
	float fRec18[2];
	float fRec9[2];
	float fRec10[2];
	float fRec2[2];
	float fRec3[2];
	float fRec32[2];
	float fRec33[2];
	float fRec28[2];
	float fRec29[2];
	float fRec23[2];
	float fRec24[2];
	
 public:
	mydsp() {
	}
	
	mydsp(const mydsp&) = default;
	
	virtual ~mydsp() = default;
	
	mydsp& operator=(const mydsp&) = default;
	
	void metadata(Meta* m) { 
		m->declare("basics.lib/name", "Faust Basic Element Library");
		m->declare("basics.lib/version", "1.22.0");
		m->declare("compile_options", "-lang cpp -i -fpga-mem-th 4 -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m->declare("filename", "main.dsp");
		m->declare("maths.lib/author", "GRAME");
		m->declare("maths.lib/copyright", "GRAME");
		m->declare("maths.lib/license", "LGPL with exception");
		m->declare("maths.lib/name", "Faust Math Library");
		m->declare("maths.lib/version", "2.9.0");
		m->declare("name", "main");
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
		fConst3 = 3.1415927f / fConst0;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(0.707f);
		fHslider1 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider2 = static_cast<FAUSTFLOAT>(5e+03f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider4 = static_cast<FAUSTFLOAT>(1e+03f);
		fHslider5 = static_cast<FAUSTFLOAT>(0.707f);
		fHslider6 = static_cast<FAUSTFLOAT>(0.707f);
		fHslider7 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider8 = static_cast<FAUSTFLOAT>(1e+02f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 2; l0 = l0 + 1) {
			fRec0[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 2; l1 = l1 + 1) {
			fRec1[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 2; l2 = l2 + 1) {
			fRec7[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 2; l3 = l3 + 1) {
			fRec8[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 2; l4 = l4 + 1) {
			fRec13[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 2; l5 = l5 + 1) {
			fRec14[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 2; l6 = l6 + 1) {
			fRec15[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 2; l7 = l7 + 1) {
			fRec16[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 2; l8 = l8 + 1) {
			fRec22[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 2; l9 = l9 + 1) {
			fRec17[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 2; l10 = l10 + 1) {
			fRec18[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 2; l11 = l11 + 1) {
			fRec9[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 2; l12 = l12 + 1) {
			fRec10[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 2; l13 = l13 + 1) {
			fRec2[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 2; l14 = l14 + 1) {
			fRec3[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 2; l15 = l15 + 1) {
			fRec32[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 2; l16 = l16 + 1) {
			fRec33[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 2; l17 = l17 + 1) {
			fRec28[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 2; l18 = l18 + 1) {
			fRec29[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 2; l19 = l19 + 1) {
			fRec23[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 2; l20 = l20 + 1) {
			fRec24[l20] = 0.0f;
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
	
	virtual mydsp* clone() {
		return new mydsp(*this);
	}
	
	virtual int getSampleRate() {
		return fSampleRate;
	}
	
	virtual void buildUserInterface(UI* ui_interface) {
		ui_interface->openVerticalBox("main");
		ui_interface->declare(&fHslider2, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ High Freq", &fHslider2, FAUSTFLOAT(5e+03f), FAUSTFLOAT(1e+03f), FAUSTFLOAT(2e+04f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider1, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ High Gain", &fHslider1, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ High Q", &fHslider0, FAUSTFLOAT(0.707f), FAUSTFLOAT(0.707f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider8, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ Low Freq", &fHslider8, FAUSTFLOAT(1e+02f), FAUSTFLOAT(2e+01f), FAUSTFLOAT(1e+03f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider7, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ Low Gain", &fHslider7, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ Low Q", &fHslider6, FAUSTFLOAT(0.707f), FAUSTFLOAT(0.707f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider4, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ Mid Freq", &fHslider4, FAUSTFLOAT(1e+03f), FAUSTFLOAT(1e+02f), FAUSTFLOAT(1e+04f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider3, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ Mid Gain", &fHslider3, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ Mid Q", &fHslider5, FAUSTFLOAT(0.707f), FAUSTFLOAT(0.707f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** RESTRICT inputs, FAUSTFLOAT** RESTRICT outputs) {
		FAUSTFLOAT* input0 = inputs[0];
		FAUSTFLOAT* input1 = inputs[1];
		FAUSTFLOAT* output0 = outputs[0];
		FAUSTFLOAT* output1 = outputs[1];
		float fSlow0 = fConst1 * static_cast<float>(fHslider0);
		float fSlow1 = fConst1 * static_cast<float>(fHslider1);
		float fSlow2 = fConst1 * static_cast<float>(fHslider2);
		float fSlow3 = fConst1 * static_cast<float>(fHslider3);
		float fSlow4 = fConst1 * static_cast<float>(fHslider4);
		float fSlow5 = fConst1 * static_cast<float>(fHslider5);
		float fSlow6 = fConst1 * static_cast<float>(fHslider6);
		float fSlow7 = fConst1 * static_cast<float>(fHslider7);
		float fSlow8 = fConst1 * static_cast<float>(fHslider8);
		for (int i0 = 0; i0 < count; i0 = i0 + 1) {
			fRec0[0] = fSlow0 + fConst2 * fRec0[1];
			fRec1[0] = fSlow1 + fConst2 * fRec1[1];
			float fTemp0 = std::pow(1e+01f, 0.05f * fRec1[0]);
			float fTemp1 = std::sqrt(fTemp0);
			fRec7[0] = fSlow2 + fConst2 * fRec7[1];
			float fTemp2 = std::tan(fConst3 * fRec7[0]);
			float fTemp3 = 1.0f / fRec0[0] + fTemp2;
			float fTemp4 = fTemp2 * fTemp3 + 1.0f;
			fRec8[0] = fSlow3 + fConst2 * fRec8[1];
			float fTemp5 = std::pow(1e+01f, 0.05f * fRec8[0]);
			fRec13[0] = fSlow4 + fConst2 * fRec13[1];
			float fTemp6 = std::tan(fConst3 * fRec13[0]);
			fRec14[0] = fSlow5 + fConst2 * fRec14[1];
			float fTemp7 = 1.0f / fRec14[0] + fTemp6;
			float fTemp8 = fTemp6 * fTemp7 + 1.0f;
			fRec15[0] = fSlow6 + fConst2 * fRec15[1];
			fRec16[0] = fSlow7 + fConst2 * fRec16[1];
			float fTemp9 = std::pow(1e+01f, 0.05f * fRec16[0]);
			float fTemp10 = std::sqrt(fTemp9);
			fRec22[0] = fSlow8 + fConst2 * fRec22[1];
			float fTemp11 = std::tan(fConst3 * fRec22[0]);
			float fTemp12 = 1.0f / fRec15[0] + fTemp11;
			float fTemp13 = fTemp11 * fTemp12 + 1.0f;
			float fTemp14 = static_cast<float>(input0[i0]) - (fTemp12 * fRec17[1] + fRec18[1]);
			float fTemp15 = fTemp11 * fTemp14 / fTemp13;
			fRec17[0] = fRec17[1] + 2.0f * fTemp15;
			float fTemp16 = fRec17[1] + fTemp15;
			float fTemp17 = fTemp11 * fTemp16;
			fRec18[0] = fRec18[1] + 2.0f * fTemp17;
			float fRec19 = fRec18[1] + fTemp17;
			float fTemp18 = fTemp14 / fTemp13;
			float fRec20 = fTemp18;
			float fRec21 = fTemp16;
			float fTemp19 = fRec20 + fRec19 * fTemp9 + fRec21 * fTemp10 / fRec15[0] - (fTemp7 * fRec9[1] + fRec10[1]);
			float fTemp20 = fTemp6 * fTemp19 / fTemp8;
			fRec9[0] = fRec9[1] + 2.0f * fTemp20;
			float fTemp21 = fRec9[1] + fTemp20;
			float fTemp22 = fTemp6 * fTemp21;
			fRec10[0] = fRec10[1] + 2.0f * fTemp22;
			float fRec11 = fTemp21;
			float fTemp23 = fTemp19 / fTemp8;
			float fRec12 = fTemp22 + fRec10[1] + fTemp23;
			float fTemp24 = fRec12 + fRec11 * fTemp5 - (fTemp3 * fRec2[1] + fRec3[1]);
			float fTemp25 = fTemp2 * fTemp24 / fTemp4;
			fRec2[0] = fRec2[1] + 2.0f * fTemp25;
			float fTemp26 = fRec2[1] + fTemp25;
			float fTemp27 = fTemp2 * fTemp26;
			fRec3[0] = fRec3[1] + 2.0f * fTemp27;
			float fRec4 = fRec3[1] + fTemp27;
			float fTemp28 = fTemp24 / fTemp4;
			float fRec5 = fTemp28;
			float fRec6 = fTemp26;
			output0[i0] = static_cast<FAUSTFLOAT>(tanhf(fRec4 + fRec5 * fTemp0 + fRec6 * fTemp1 / fRec0[0]));
			float fTemp29 = static_cast<float>(input1[i0]) - (fTemp12 * fRec32[1] + fRec33[1]);
			float fTemp30 = fTemp11 * fTemp29 / fTemp13;
			fRec32[0] = fRec32[1] + 2.0f * fTemp30;
			float fTemp31 = fRec32[1] + fTemp30;
			float fTemp32 = fTemp11 * fTemp31;
			fRec33[0] = fRec33[1] + 2.0f * fTemp32;
			float fRec34 = fRec33[1] + fTemp32;
			float fTemp33 = fTemp29 / fTemp13;
			float fRec35 = fTemp33;
			float fRec36 = fTemp31;
			float fTemp34 = fRec35 + fRec34 * fTemp9 + fRec36 * fTemp10 / fRec15[0] - (fTemp7 * fRec28[1] + fRec29[1]);
			float fTemp35 = fTemp6 * fTemp34 / fTemp8;
			fRec28[0] = fRec28[1] + 2.0f * fTemp35;
			float fTemp36 = fRec28[1] + fTemp35;
			float fTemp37 = fTemp6 * fTemp36;
			fRec29[0] = fRec29[1] + 2.0f * fTemp37;
			float fRec30 = fTemp36;
			float fTemp38 = fTemp34 / fTemp8;
			float fRec31 = fTemp37 + fRec29[1] + fTemp38;
			float fTemp39 = fRec31 + fRec30 * fTemp5 - (fTemp3 * fRec23[1] + fRec24[1]);
			float fTemp40 = fTemp2 * fTemp39 / fTemp4;
			fRec23[0] = fRec23[1] + 2.0f * fTemp40;
			float fTemp41 = fRec23[1] + fTemp40;
			float fTemp42 = fTemp2 * fTemp41;
			fRec24[0] = fRec24[1] + 2.0f * fTemp42;
			float fRec25 = fRec24[1] + fTemp42;
			float fTemp43 = fTemp39 / fTemp4;
			float fRec26 = fTemp43;
			float fRec27 = fTemp41;
			output1[i0] = static_cast<FAUSTFLOAT>(tanhf(fRec25 + fRec26 * fTemp0 + fRec27 * fTemp1 / fRec0[0]));
			fRec0[1] = fRec0[0];
			fRec1[1] = fRec1[0];
			fRec7[1] = fRec7[0];
			fRec8[1] = fRec8[0];
			fRec13[1] = fRec13[0];
			fRec14[1] = fRec14[0];
			fRec15[1] = fRec15[0];
			fRec16[1] = fRec16[0];
			fRec22[1] = fRec22[0];
			fRec17[1] = fRec17[0];
			fRec18[1] = fRec18[0];
			fRec9[1] = fRec9[0];
			fRec10[1] = fRec10[0];
			fRec2[1] = fRec2[0];
			fRec3[1] = fRec3[0];
			fRec32[1] = fRec32[0];
			fRec33[1] = fRec33[0];
			fRec28[1] = fRec28[0];
			fRec29[1] = fRec29[0];
			fRec23[1] = fRec23[0];
			fRec24[1] = fRec24[0];
		}
	}

};

#endif
