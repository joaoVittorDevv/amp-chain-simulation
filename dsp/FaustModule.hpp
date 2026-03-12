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

static float mydsp_faustpower2_f(float value) {
	return value * value;
}

class mydsp : public dsp {
	
 private:
	
	int fSampleRate;
	float fConst0;
	float fConst1;
	float fConst2;
	FAUSTFLOAT fHslider0;
	float fRec0[2];
	float fConst3;
	FAUSTFLOAT fHslider1;
	float fRec1[2];
	float fConst4;
	FAUSTFLOAT fHslider2;
	float fRec2[2];
	FAUSTFLOAT fHslider3;
	float fRec4[2];
	FAUSTFLOAT fHslider4;
	float fRec5[2];
	FAUSTFLOAT fHslider5;
	float fRec6[2];
	FAUSTFLOAT fHslider6;
	float fRec8[2];
	FAUSTFLOAT fHslider7;
	float fRec9[2];
	FAUSTFLOAT fHslider8;
	float fRec10[2];
	float fRec11[3];
	float fRec7[3];
	float fRec3[3];
	float fRec14[3];
	float fRec13[3];
	float fRec12[3];
	
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
		m->declare("filters.lib/fir:author", "Julius O. Smith III");
		m->declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/iir:author", "Julius O. Smith III");
		m->declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/name", "Faust Filters Library");
		m->declare("filters.lib/peak_eq:author", "Julius O. Smith III");
		m->declare("filters.lib/peak_eq:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/peak_eq:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/tf2:author", "Julius O. Smith III");
		m->declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/tf2s:author", "Julius O. Smith III");
		m->declare("filters.lib/tf2s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf2s:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/version", "1.7.1");
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
		fConst4 = 6.2831855f / fConst0;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(5e+03f);
		fHslider1 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider2 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider3 = static_cast<FAUSTFLOAT>(1e+03f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider5 = static_cast<FAUSTFLOAT>(1.0f);
		fHslider6 = static_cast<FAUSTFLOAT>(1e+02f);
		fHslider7 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider8 = static_cast<FAUSTFLOAT>(1.0f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 2; l0 = l0 + 1) {
			fRec0[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 2; l1 = l1 + 1) {
			fRec1[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 2; l2 = l2 + 1) {
			fRec2[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 2; l3 = l3 + 1) {
			fRec4[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 2; l4 = l4 + 1) {
			fRec5[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 2; l5 = l5 + 1) {
			fRec6[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 2; l6 = l6 + 1) {
			fRec8[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 2; l7 = l7 + 1) {
			fRec9[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 2; l8 = l8 + 1) {
			fRec10[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 3; l9 = l9 + 1) {
			fRec11[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 3; l10 = l10 + 1) {
			fRec7[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 3; l11 = l11 + 1) {
			fRec3[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 3; l12 = l12 + 1) {
			fRec14[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 3; l13 = l13 + 1) {
			fRec13[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 3; l14 = l14 + 1) {
			fRec12[l14] = 0.0f;
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
		ui_interface->declare(&fHslider0, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ High Freq", &fHslider0, FAUSTFLOAT(5e+03f), FAUSTFLOAT(1e+03f), FAUSTFLOAT(2e+04f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider1, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ High Gain", &fHslider1, FAUSTFLOAT(0.0f), FAUSTFLOAT(-24.0f), FAUSTFLOAT(24.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ High Q", &fHslider2, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.1f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider6, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ Low Freq", &fHslider6, FAUSTFLOAT(1e+02f), FAUSTFLOAT(2e+01f), FAUSTFLOAT(1e+03f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider7, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ Low Gain", &fHslider7, FAUSTFLOAT(0.0f), FAUSTFLOAT(-24.0f), FAUSTFLOAT(24.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ Low Q", &fHslider8, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.1f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider3, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ Mid Freq", &fHslider3, FAUSTFLOAT(1e+03f), FAUSTFLOAT(1e+02f), FAUSTFLOAT(1e+04f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider4, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ Mid Gain", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(-24.0f), FAUSTFLOAT(24.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ Mid Q", &fHslider5, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.1f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
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
			float fTemp0 = std::tan(fConst3 * fRec0[0]);
			fRec1[0] = fSlow1 + fConst2 * fRec1[1];
			int iTemp1 = fRec1[0] > 0.0f;
			float fTemp2 = std::sin(fConst4 * fRec0[0]);
			fRec2[0] = fSlow2 + fConst2 * fRec2[1];
			float fTemp3 = fConst3 * (fRec2[0] * std::pow(1e+01f, 0.05f * std::fabs(fRec1[0])) / fTemp2);
			float fTemp4 = fConst3 * (fRec2[0] / fTemp2);
			float fTemp5 = ((iTemp1) ? fTemp4 : fTemp3);
			float fTemp6 = 1.0f / fTemp0;
			float fTemp7 = (fTemp6 + fTemp5) / fTemp0 + 1.0f;
			float fTemp8 = ((iTemp1) ? fTemp3 : fTemp4);
			float fTemp9 = (fTemp6 - fTemp8) / fTemp0 + 1.0f;
			float fTemp10 = 1.0f - 1.0f / mydsp_faustpower2_f(fTemp0);
			float fTemp11 = 2.0f * fRec3[1] * fTemp10;
			float fTemp12 = (fTemp6 - fTemp5) / fTemp0 + 1.0f;
			fRec4[0] = fSlow3 + fConst2 * fRec4[1];
			float fTemp13 = std::tan(fConst3 * fRec4[0]);
			fRec5[0] = fSlow4 + fConst2 * fRec5[1];
			int iTemp14 = fRec5[0] > 0.0f;
			float fTemp15 = std::sin(fConst4 * fRec4[0]);
			fRec6[0] = fSlow5 + fConst2 * fRec6[1];
			float fTemp16 = fConst3 * (fRec6[0] * std::pow(1e+01f, 0.05f * std::fabs(fRec5[0])) / fTemp15);
			float fTemp17 = fConst3 * (fRec6[0] / fTemp15);
			float fTemp18 = ((iTemp14) ? fTemp17 : fTemp16);
			float fTemp19 = 1.0f / fTemp13;
			float fTemp20 = (fTemp19 + fTemp18) / fTemp13 + 1.0f;
			float fTemp21 = ((iTemp14) ? fTemp16 : fTemp17);
			float fTemp22 = (fTemp19 - fTemp21) / fTemp13 + 1.0f;
			float fTemp23 = 1.0f - 1.0f / mydsp_faustpower2_f(fTemp13);
			float fTemp24 = 2.0f * fRec7[1] * fTemp23;
			float fTemp25 = (fTemp19 - fTemp18) / fTemp13 + 1.0f;
			fRec8[0] = fSlow6 + fConst2 * fRec8[1];
			float fTemp26 = std::tan(fConst3 * fRec8[0]);
			fRec9[0] = fSlow7 + fConst2 * fRec9[1];
			int iTemp27 = fRec9[0] > 0.0f;
			float fTemp28 = std::sin(fConst4 * fRec8[0]);
			fRec10[0] = fSlow8 + fConst2 * fRec10[1];
			float fTemp29 = fConst3 * (fRec10[0] * std::pow(1e+01f, 0.05f * std::fabs(fRec9[0])) / fTemp28);
			float fTemp30 = fConst3 * (fRec10[0] / fTemp28);
			float fTemp31 = ((iTemp27) ? fTemp30 : fTemp29);
			float fTemp32 = 1.0f / fTemp26;
			float fTemp33 = (fTemp32 + fTemp31) / fTemp26 + 1.0f;
			float fTemp34 = ((iTemp27) ? fTemp29 : fTemp30);
			float fTemp35 = (fTemp32 - fTemp34) / fTemp26 + 1.0f;
			float fTemp36 = 1.0f - 1.0f / mydsp_faustpower2_f(fTemp26);
			float fTemp37 = 2.0f * fRec11[1] * fTemp36;
			float fTemp38 = (fTemp32 - fTemp31) / fTemp26 + 1.0f;
			fRec11[0] = static_cast<float>(input0[i0]) - (fRec11[2] * fTemp38 + fTemp37) / fTemp33;
			float fTemp39 = (fTemp32 + fTemp34) / fTemp26 + 1.0f;
			fRec7[0] = (fTemp37 + fRec11[0] * fTemp39 + fRec11[2] * fTemp35) / fTemp33 - (fRec7[2] * fTemp25 + fTemp24) / fTemp20;
			float fTemp40 = (fTemp19 + fTemp21) / fTemp13 + 1.0f;
			fRec3[0] = (fTemp24 + fRec7[0] * fTemp40 + fRec7[2] * fTemp22) / fTemp20 - (fRec3[2] * fTemp12 + fTemp11) / fTemp7;
			float fTemp41 = (fTemp6 + fTemp8) / fTemp0 + 1.0f;
			output0[i0] = static_cast<FAUSTFLOAT>((fTemp11 + fRec3[0] * fTemp41 + fRec3[2] * fTemp9) / fTemp7);
			float fTemp42 = 2.0f * fTemp10 * fRec12[1];
			float fTemp43 = 2.0f * fTemp23 * fRec13[1];
			float fTemp44 = 2.0f * fTemp36 * fRec14[1];
			fRec14[0] = static_cast<float>(input1[i0]) - (fTemp38 * fRec14[2] + fTemp44) / fTemp33;
			fRec13[0] = (fTemp44 + fRec14[0] * fTemp39 + fTemp35 * fRec14[2]) / fTemp33 - (fTemp25 * fRec13[2] + fTemp43) / fTemp20;
			fRec12[0] = (fTemp43 + fRec13[0] * fTemp40 + fTemp22 * fRec13[2]) / fTemp20 - (fTemp12 * fRec12[2] + fTemp42) / fTemp7;
			output1[i0] = static_cast<FAUSTFLOAT>((fTemp42 + fRec12[0] * fTemp41 + fTemp9 * fRec12[2]) / fTemp7);
			fRec0[1] = fRec0[0];
			fRec1[1] = fRec1[0];
			fRec2[1] = fRec2[0];
			fRec4[1] = fRec4[0];
			fRec5[1] = fRec5[0];
			fRec6[1] = fRec6[0];
			fRec8[1] = fRec8[0];
			fRec9[1] = fRec9[0];
			fRec10[1] = fRec10[0];
			fRec11[2] = fRec11[1];
			fRec11[1] = fRec11[0];
			fRec7[2] = fRec7[1];
			fRec7[1] = fRec7[0];
			fRec3[2] = fRec3[1];
			fRec3[1] = fRec3[0];
			fRec14[2] = fRec14[1];
			fRec14[1] = fRec14[0];
			fRec13[2] = fRec13[1];
			fRec13[1] = fRec13[0];
			fRec12[2] = fRec12[1];
			fRec12[1] = fRec12[0];
		}
	}

};

#endif
