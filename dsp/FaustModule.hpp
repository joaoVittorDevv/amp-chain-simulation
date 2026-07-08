/* ------------------------------------------------------------
name: "main"
Code generated with Faust 2.85.1 (https://faust.grame.fr)
Compilation options: -lang cpp -i -fpga-mem-th 4 -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0 -vec -lv 0 -vs 32
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
	float fRec5_perm[4];
	FAUSTFLOAT fHslider1;
	float fRec10_perm[4];
	FAUSTFLOAT fHslider2;
	float fRec16_perm[4];
	FAUSTFLOAT fHslider3;
	float fRec17_perm[4];
	float fConst3;
	float fRec11_perm[4];
	float fRec12_perm[4];
	FAUSTFLOAT fHslider4;
	float fRec18_perm[4];
	FAUSTFLOAT fHslider5;
	float fRec19_perm[4];
	float fRec6_perm[4];
	float fRec7_perm[4];
	FAUSTFLOAT fHslider6;
	float fRec20_perm[4];
	FAUSTFLOAT fHslider7;
	float fRec21_perm[4];
	float fRec0_perm[4];
	float fRec1_perm[4];
	FAUSTFLOAT fHslider8;
	float fRec22_perm[4];
	FAUSTFLOAT fCheckbox0;
	float fRec32_perm[4];
	float fRec33_perm[4];
	float fRec28_perm[4];
	float fRec29_perm[4];
	float fRec23_perm[4];
	float fRec24_perm[4];
	
 public:
	mydsp() {
	}
	
	mydsp(const mydsp&) = default;
	
	virtual ~mydsp() = default;
	
	mydsp& operator=(const mydsp&) = default;
	
	void metadata(Meta* m) { 
		m->declare("basics.lib/name", "Faust Basic Element Library");
		m->declare("basics.lib/version", "1.22.0");
		m->declare("compile_options", "-lang cpp -i -fpga-mem-th 4 -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0 -vec -lv 0 -vs 32");
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
		fHslider0 = static_cast<FAUSTFLOAT>(5e+03f);
		fHslider1 = static_cast<FAUSTFLOAT>(1e+03f);
		fHslider2 = static_cast<FAUSTFLOAT>(1e+02f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.707f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider5 = static_cast<FAUSTFLOAT>(0.707f);
		fHslider6 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider7 = static_cast<FAUSTFLOAT>(0.707f);
		fHslider8 = static_cast<FAUSTFLOAT>(0.0f);
		fCheckbox0 = static_cast<FAUSTFLOAT>(0.0f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 4; l0 = l0 + 1) {
			fRec5_perm[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 4; l1 = l1 + 1) {
			fRec10_perm[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 4; l2 = l2 + 1) {
			fRec16_perm[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 4; l3 = l3 + 1) {
			fRec17_perm[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 4; l4 = l4 + 1) {
			fRec11_perm[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 4; l5 = l5 + 1) {
			fRec12_perm[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 4; l6 = l6 + 1) {
			fRec18_perm[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 4; l7 = l7 + 1) {
			fRec19_perm[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 4; l8 = l8 + 1) {
			fRec6_perm[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 4; l9 = l9 + 1) {
			fRec7_perm[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 4; l10 = l10 + 1) {
			fRec20_perm[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 4; l11 = l11 + 1) {
			fRec21_perm[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 4; l12 = l12 + 1) {
			fRec0_perm[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 4; l13 = l13 + 1) {
			fRec1_perm[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 4; l14 = l14 + 1) {
			fRec22_perm[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 4; l15 = l15 + 1) {
			fRec32_perm[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 4; l16 = l16 + 1) {
			fRec33_perm[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 4; l17 = l17 + 1) {
			fRec28_perm[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 4; l18 = l18 + 1) {
			fRec29_perm[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 4; l19 = l19 + 1) {
			fRec23_perm[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 4; l20 = l20 + 1) {
			fRec24_perm[l20] = 0.0f;
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
		ui_interface->declare(&fHslider8, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ High Gain", &fHslider8, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ High Q", &fHslider7, FAUSTFLOAT(0.707f), FAUSTFLOAT(0.707f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider2, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ Low Freq", &fHslider2, FAUSTFLOAT(1e+02f), FAUSTFLOAT(2e+01f), FAUSTFLOAT(1e+03f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider4, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ Low Gain", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ Low Q", &fHslider3, FAUSTFLOAT(0.707f), FAUSTFLOAT(0.707f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider1, "unit", "Hz");
		ui_interface->addHorizontalSlider("EQ Mid Freq", &fHslider1, FAUSTFLOAT(1e+03f), FAUSTFLOAT(1e+02f), FAUSTFLOAT(1e+04f), FAUSTFLOAT(1.0f));
		ui_interface->declare(&fHslider6, "unit", "dB");
		ui_interface->addHorizontalSlider("EQ Mid Gain", &fHslider6, FAUSTFLOAT(0.0f), FAUSTFLOAT(-12.0f), FAUSTFLOAT(12.0f), FAUSTFLOAT(0.1f));
		ui_interface->addHorizontalSlider("EQ Mid Q", &fHslider5, FAUSTFLOAT(0.707f), FAUSTFLOAT(0.707f), FAUSTFLOAT(1e+01f), FAUSTFLOAT(0.01f));
		ui_interface->addCheckButton("EQ Tanh Bypass", &fCheckbox0);
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** RESTRICT inputs, FAUSTFLOAT** RESTRICT outputs) {
		FAUSTFLOAT* input0_ptr = inputs[0];
		FAUSTFLOAT* input1_ptr = inputs[1];
		FAUSTFLOAT* output0_ptr = outputs[0];
		FAUSTFLOAT* output1_ptr = outputs[1];
		float fSlow0 = fConst1 * static_cast<float>(fHslider0);
		float fRec5_tmp[36];
		float* fRec5 = &fRec5_tmp[4];
		float fSlow1 = fConst1 * static_cast<float>(fHslider1);
		float fRec10_tmp[36];
		float* fRec10 = &fRec10_tmp[4];
		float fSlow2 = fConst1 * static_cast<float>(fHslider2);
		float fRec16_tmp[36];
		float* fRec16 = &fRec16_tmp[4];
		float fSlow3 = fConst1 * static_cast<float>(fHslider3);
		float fRec17_tmp[36];
		float* fRec17 = &fRec17_tmp[4];
		float fZec0[32];
		float fZec1[32];
		float fZec2[32];
		float fZec3[32];
		float fZec4[32];
		float fRec11_tmp[36];
		float* fRec11 = &fRec11_tmp[4];
		float fZec5[32];
		float fZec6[32];
		float fRec12_tmp[36];
		float* fRec12 = &fRec12_tmp[4];
		float fRec13[32];
		float fZec7[32];
		float fRec14[32];
		float fRec15[32];
		float fSlow4 = fConst1 * static_cast<float>(fHslider4);
		float fRec18_tmp[36];
		float* fRec18 = &fRec18_tmp[4];
		float fSlow5 = fConst1 * static_cast<float>(fHslider5);
		float fRec19_tmp[36];
		float* fRec19 = &fRec19_tmp[4];
		float fZec8[32];
		float fZec9[32];
		float fZec10[32];
		float fZec11[32];
		float fZec12[32];
		float fZec13[32];
		float fZec14[32];
		float fRec6_tmp[36];
		float* fRec6 = &fRec6_tmp[4];
		float fZec15[32];
		float fZec16[32];
		float fRec7_tmp[36];
		float* fRec7 = &fRec7_tmp[4];
		float fRec8[32];
		float fZec17[32];
		float fRec9[32];
		float fSlow6 = fConst1 * static_cast<float>(fHslider6);
		float fRec20_tmp[36];
		float* fRec20 = &fRec20_tmp[4];
		float fSlow7 = fConst1 * static_cast<float>(fHslider7);
		float fRec21_tmp[36];
		float* fRec21 = &fRec21_tmp[4];
		float fZec18[32];
		float fZec19[32];
		float fZec20[32];
		float fZec21[32];
		float fZec22[32];
		float fZec23[32];
		float fRec0_tmp[36];
		float* fRec0 = &fRec0_tmp[4];
		float fZec24[32];
		float fZec25[32];
		float fRec1_tmp[36];
		float* fRec1 = &fRec1_tmp[4];
		float fRec2[32];
		float fZec26[32];
		float fRec3[32];
		float fRec4[32];
		float fSlow8 = fConst1 * static_cast<float>(fHslider8);
		float fRec22_tmp[36];
		float* fRec22 = &fRec22_tmp[4];
		float fZec27[32];
		float fZec28[32];
		float fZec29[32];
		float fSlow9 = static_cast<float>(fCheckbox0);
		float fSlow10 = 1.0f - fSlow9;
		float fZec30[32];
		float fZec31[32];
		float fRec32_tmp[36];
		float* fRec32 = &fRec32_tmp[4];
		float fZec32[32];
		float fZec33[32];
		float fRec33_tmp[36];
		float* fRec33 = &fRec33_tmp[4];
		float fRec34[32];
		float fZec34[32];
		float fRec35[32];
		float fRec36[32];
		float fZec35[32];
		float fZec36[32];
		float fRec28_tmp[36];
		float* fRec28 = &fRec28_tmp[4];
		float fZec37[32];
		float fZec38[32];
		float fRec29_tmp[36];
		float* fRec29 = &fRec29_tmp[4];
		float fRec30[32];
		float fZec39[32];
		float fRec31[32];
		float fZec40[32];
		float fZec41[32];
		float fRec23_tmp[36];
		float* fRec23 = &fRec23_tmp[4];
		float fZec42[32];
		float fZec43[32];
		float fRec24_tmp[36];
		float* fRec24 = &fRec24_tmp[4];
		float fRec25[32];
		float fZec44[32];
		float fRec26[32];
		float fRec27[32];
		float fZec45[32];
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
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec16_tmp[j4] = fRec16_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec16[i] = fSlow2 + fConst2 * fRec16[i - 1];
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec16_perm[j5] = fRec16_tmp[vsize + j5];
			}
			/* Recursive loop 1 */
			/* Pre code */
			for (int j2 = 0; j2 < 4; j2 = j2 + 1) {
				fRec10_tmp[j2] = fRec10_perm[j2];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec10[i] = fSlow1 + fConst2 * fRec10[i - 1];
			}
			/* Post code */
			for (int j3 = 0; j3 < 4; j3 = j3 + 1) {
				fRec10_perm[j3] = fRec10_tmp[vsize + j3];
			}
			/* Recursive loop 2 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec17_tmp[j6] = fRec17_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec17[i] = fSlow3 + fConst2 * fRec17[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec17_perm[j7] = fRec17_tmp[vsize + j7];
			}
			/* Vectorizable loop 3 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::tan(fConst3 * fRec16[i]);
			}
			/* Recursive loop 4 */
			/* Pre code */
			for (int j0 = 0; j0 < 4; j0 = j0 + 1) {
				fRec5_tmp[j0] = fRec5_perm[j0];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec5[i] = fSlow0 + fConst2 * fRec5[i - 1];
			}
			/* Post code */
			for (int j1 = 0; j1 < 4; j1 = j1 + 1) {
				fRec5_perm[j1] = fRec5_tmp[vsize + j1];
			}
			/* Vectorizable loop 5 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec1[i] = 1.0f / fRec17[i] + fZec0[i];
			}
			/* Recursive loop 6 */
			/* Pre code */
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec18_tmp[j12] = fRec18_perm[j12];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec18[i] = fSlow4 + fConst2 * fRec18[i - 1];
			}
			/* Post code */
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec18_perm[j13] = fRec18_tmp[vsize + j13];
			}
			/* Recursive loop 7 */
			/* Pre code */
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec19_tmp[j14] = fRec19_perm[j14];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec19[i] = fSlow5 + fConst2 * fRec19[i - 1];
			}
			/* Post code */
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec19_perm[j15] = fRec19_tmp[vsize + j15];
			}
			/* Vectorizable loop 8 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec8[i] = std::tan(fConst3 * fRec10[i]);
			}
			/* Vectorizable loop 9 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec2[i] = fZec0[i] * fZec1[i] + 1.0f;
			}
			/* Vectorizable loop 10 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = 1.0f / fRec19[i] + fZec8[i];
			}
			/* Vectorizable loop 11 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = std::pow(1e+01f, 0.05f * fRec18[i]);
			}
			/* Recursive loop 12 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec21_tmp[j22] = fRec21_perm[j22];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec21[i] = fSlow7 + fConst2 * fRec21[i - 1];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec21_perm[j23] = fRec21_tmp[vsize + j23];
			}
			/* Vectorizable loop 13 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec18[i] = std::tan(fConst3 * fRec5[i]);
			}
			/* Recursive loop 14 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec11_tmp[j8] = fRec11_perm[j8];
			}
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec12_tmp[j10] = fRec12_perm[j10];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec3[i] = static_cast<float>(input0[i]) - (fZec1[i] * fRec11[i - 1] + fRec12[i - 1]);
				fZec4[i] = fZec0[i] * fZec3[i] / fZec2[i];
				fRec11[i] = fRec11[i - 1] + 2.0f * fZec4[i];
				fZec5[i] = fRec11[i - 1] + fZec4[i];
				fZec6[i] = fZec0[i] * fZec5[i];
				fRec12[i] = fRec12[i - 1] + 2.0f * fZec6[i];
				fRec13[i] = fRec12[i - 1] + fZec6[i];
				fZec7[i] = fZec3[i] / fZec2[i];
				fRec14[i] = fZec7[i];
				fRec15[i] = fZec5[i];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec11_perm[j9] = fRec11_tmp[vsize + j9];
			}
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec12_perm[j11] = fRec12_tmp[vsize + j11];
			}
			/* Vectorizable loop 15 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = fZec8[i] * fZec9[i] + 1.0f;
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = std::sqrt(fZec11[i]);
			}
			/* Recursive loop 17 */
			/* Pre code */
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec20_tmp[j20] = fRec20_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec20[i] = fSlow6 + fConst2 * fRec20[i - 1];
			}
			/* Post code */
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec20_perm[j21] = fRec20_tmp[vsize + j21];
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec19[i] = 1.0f / fRec21[i] + fZec18[i];
			}
			/* Recursive loop 19 */
			/* Pre code */
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec22_tmp[j28] = fRec22_perm[j28];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec22[i] = fSlow8 + fConst2 * fRec22[i - 1];
			}
			/* Post code */
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec22_perm[j29] = fRec22_tmp[vsize + j29];
			}
			/* Recursive loop 20 */
			/* Pre code */
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec32_tmp[j30] = fRec32_perm[j30];
			}
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec33_tmp[j32] = fRec33_perm[j32];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec30[i] = static_cast<float>(input1[i]) - (fZec1[i] * fRec32[i - 1] + fRec33[i - 1]);
				fZec31[i] = fZec0[i] * fZec30[i] / fZec2[i];
				fRec32[i] = fRec32[i - 1] + 2.0f * fZec31[i];
				fZec32[i] = fRec32[i - 1] + fZec31[i];
				fZec33[i] = fZec0[i] * fZec32[i];
				fRec33[i] = fRec33[i - 1] + 2.0f * fZec33[i];
				fRec34[i] = fRec33[i - 1] + fZec33[i];
				fZec34[i] = fZec30[i] / fZec2[i];
				fRec35[i] = fZec34[i];
				fRec36[i] = fZec32[i];
			}
			/* Post code */
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec32_perm[j31] = fRec32_tmp[vsize + j31];
			}
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec33_perm[j33] = fRec33_tmp[vsize + j33];
			}
			/* Recursive loop 21 */
			/* Pre code */
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec6_tmp[j16] = fRec6_perm[j16];
			}
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec7_tmp[j18] = fRec7_perm[j18];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec13[i] = fRec14[i] + fRec13[i] * fZec11[i] + fRec15[i] * fZec12[i] / fRec17[i] - (fZec9[i] * fRec6[i - 1] + fRec7[i - 1]);
				fZec14[i] = fZec8[i] * fZec13[i] / fZec10[i];
				fRec6[i] = fRec6[i - 1] + 2.0f * fZec14[i];
				fZec15[i] = fRec6[i - 1] + fZec14[i];
				fZec16[i] = fZec8[i] * fZec15[i];
				fRec7[i] = fRec7[i - 1] + 2.0f * fZec16[i];
				fRec8[i] = fZec15[i];
				fZec17[i] = fZec13[i] / fZec10[i];
				fRec9[i] = fZec16[i] + fRec7[i - 1] + fZec17[i];
			}
			/* Post code */
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec6_perm[j17] = fRec6_tmp[vsize + j17];
			}
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec7_perm[j19] = fRec7_tmp[vsize + j19];
			}
			/* Vectorizable loop 22 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec20[i] = fZec18[i] * fZec19[i] + 1.0f;
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec21[i] = std::pow(1e+01f, 0.05f * fRec20[i]);
			}
			/* Vectorizable loop 24 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec27[i] = std::pow(1e+01f, 0.05f * fRec22[i]);
			}
			/* Recursive loop 25 */
			/* Pre code */
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec28_tmp[j34] = fRec28_perm[j34];
			}
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec29_tmp[j36] = fRec29_perm[j36];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec35[i] = fRec35[i] + fRec34[i] * fZec11[i] + fRec36[i] * fZec12[i] / fRec17[i] - (fZec9[i] * fRec28[i - 1] + fRec29[i - 1]);
				fZec36[i] = fZec8[i] * fZec35[i] / fZec10[i];
				fRec28[i] = fRec28[i - 1] + 2.0f * fZec36[i];
				fZec37[i] = fRec28[i - 1] + fZec36[i];
				fZec38[i] = fZec8[i] * fZec37[i];
				fRec29[i] = fRec29[i - 1] + 2.0f * fZec38[i];
				fRec30[i] = fZec37[i];
				fZec39[i] = fZec35[i] / fZec10[i];
				fRec31[i] = fZec38[i] + fRec29[i - 1] + fZec39[i];
			}
			/* Post code */
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec28_perm[j35] = fRec28_tmp[vsize + j35];
			}
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec29_perm[j37] = fRec29_tmp[vsize + j37];
			}
			/* Recursive loop 26 */
			/* Pre code */
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec0_tmp[j24] = fRec0_perm[j24];
			}
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec1_tmp[j26] = fRec1_perm[j26];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec22[i] = fRec9[i] + fRec8[i] * fZec21[i] - (fZec19[i] * fRec0[i - 1] + fRec1[i - 1]);
				fZec23[i] = fZec18[i] * fZec22[i] / fZec20[i];
				fRec0[i] = fRec0[i - 1] + 2.0f * fZec23[i];
				fZec24[i] = fRec0[i - 1] + fZec23[i];
				fZec25[i] = fZec18[i] * fZec24[i];
				fRec1[i] = fRec1[i - 1] + 2.0f * fZec25[i];
				fRec2[i] = fRec1[i - 1] + fZec25[i];
				fZec26[i] = fZec22[i] / fZec20[i];
				fRec3[i] = fZec26[i];
				fRec4[i] = fZec24[i];
			}
			/* Post code */
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec0_perm[j25] = fRec0_tmp[vsize + j25];
			}
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec1_perm[j27] = fRec1_tmp[vsize + j27];
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec28[i] = std::sqrt(fZec27[i]);
			}
			/* Recursive loop 28 */
			/* Pre code */
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec23_tmp[j38] = fRec23_perm[j38];
			}
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec24_tmp[j40] = fRec24_perm[j40];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec40[i] = fRec31[i] + fRec30[i] * fZec21[i] - (fZec19[i] * fRec23[i - 1] + fRec24[i - 1]);
				fZec41[i] = fZec18[i] * fZec40[i] / fZec20[i];
				fRec23[i] = fRec23[i - 1] + 2.0f * fZec41[i];
				fZec42[i] = fRec23[i - 1] + fZec41[i];
				fZec43[i] = fZec18[i] * fZec42[i];
				fRec24[i] = fRec24[i - 1] + 2.0f * fZec43[i];
				fRec25[i] = fRec24[i - 1] + fZec43[i];
				fZec44[i] = fZec40[i] / fZec20[i];
				fRec26[i] = fZec44[i];
				fRec27[i] = fZec42[i];
			}
			/* Post code */
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec23_perm[j39] = fRec23_tmp[vsize + j39];
			}
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec24_perm[j41] = fRec24_tmp[vsize + j41];
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec29[i] = fRec2[i] + fRec3[i] * fZec27[i] + fRec4[i] * fZec28[i] / fRec21[i];
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec45[i] = fRec25[i] + fRec26[i] * fZec27[i] + fRec27[i] * fZec28[i] / fRec21[i];
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fSlow9 * fZec29[i] + fSlow10 * tanhf(fZec29[i]));
			}
			/* Vectorizable loop 32 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fSlow9 * fZec45[i] + fSlow10 * tanhf(fZec45[i]));
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
			for (int j4 = 0; j4 < 4; j4 = j4 + 1) {
				fRec16_tmp[j4] = fRec16_perm[j4];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec16[i] = fSlow2 + fConst2 * fRec16[i - 1];
			}
			/* Post code */
			for (int j5 = 0; j5 < 4; j5 = j5 + 1) {
				fRec16_perm[j5] = fRec16_tmp[vsize + j5];
			}
			/* Recursive loop 1 */
			/* Pre code */
			for (int j2 = 0; j2 < 4; j2 = j2 + 1) {
				fRec10_tmp[j2] = fRec10_perm[j2];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec10[i] = fSlow1 + fConst2 * fRec10[i - 1];
			}
			/* Post code */
			for (int j3 = 0; j3 < 4; j3 = j3 + 1) {
				fRec10_perm[j3] = fRec10_tmp[vsize + j3];
			}
			/* Recursive loop 2 */
			/* Pre code */
			for (int j6 = 0; j6 < 4; j6 = j6 + 1) {
				fRec17_tmp[j6] = fRec17_perm[j6];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec17[i] = fSlow3 + fConst2 * fRec17[i - 1];
			}
			/* Post code */
			for (int j7 = 0; j7 < 4; j7 = j7 + 1) {
				fRec17_perm[j7] = fRec17_tmp[vsize + j7];
			}
			/* Vectorizable loop 3 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec0[i] = std::tan(fConst3 * fRec16[i]);
			}
			/* Recursive loop 4 */
			/* Pre code */
			for (int j0 = 0; j0 < 4; j0 = j0 + 1) {
				fRec5_tmp[j0] = fRec5_perm[j0];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec5[i] = fSlow0 + fConst2 * fRec5[i - 1];
			}
			/* Post code */
			for (int j1 = 0; j1 < 4; j1 = j1 + 1) {
				fRec5_perm[j1] = fRec5_tmp[vsize + j1];
			}
			/* Vectorizable loop 5 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec1[i] = 1.0f / fRec17[i] + fZec0[i];
			}
			/* Recursive loop 6 */
			/* Pre code */
			for (int j12 = 0; j12 < 4; j12 = j12 + 1) {
				fRec18_tmp[j12] = fRec18_perm[j12];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec18[i] = fSlow4 + fConst2 * fRec18[i - 1];
			}
			/* Post code */
			for (int j13 = 0; j13 < 4; j13 = j13 + 1) {
				fRec18_perm[j13] = fRec18_tmp[vsize + j13];
			}
			/* Recursive loop 7 */
			/* Pre code */
			for (int j14 = 0; j14 < 4; j14 = j14 + 1) {
				fRec19_tmp[j14] = fRec19_perm[j14];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec19[i] = fSlow5 + fConst2 * fRec19[i - 1];
			}
			/* Post code */
			for (int j15 = 0; j15 < 4; j15 = j15 + 1) {
				fRec19_perm[j15] = fRec19_tmp[vsize + j15];
			}
			/* Vectorizable loop 8 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec8[i] = std::tan(fConst3 * fRec10[i]);
			}
			/* Vectorizable loop 9 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec2[i] = fZec0[i] * fZec1[i] + 1.0f;
			}
			/* Vectorizable loop 10 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec9[i] = 1.0f / fRec19[i] + fZec8[i];
			}
			/* Vectorizable loop 11 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec11[i] = std::pow(1e+01f, 0.05f * fRec18[i]);
			}
			/* Recursive loop 12 */
			/* Pre code */
			for (int j22 = 0; j22 < 4; j22 = j22 + 1) {
				fRec21_tmp[j22] = fRec21_perm[j22];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec21[i] = fSlow7 + fConst2 * fRec21[i - 1];
			}
			/* Post code */
			for (int j23 = 0; j23 < 4; j23 = j23 + 1) {
				fRec21_perm[j23] = fRec21_tmp[vsize + j23];
			}
			/* Vectorizable loop 13 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec18[i] = std::tan(fConst3 * fRec5[i]);
			}
			/* Recursive loop 14 */
			/* Pre code */
			for (int j8 = 0; j8 < 4; j8 = j8 + 1) {
				fRec11_tmp[j8] = fRec11_perm[j8];
			}
			for (int j10 = 0; j10 < 4; j10 = j10 + 1) {
				fRec12_tmp[j10] = fRec12_perm[j10];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec3[i] = static_cast<float>(input0[i]) - (fZec1[i] * fRec11[i - 1] + fRec12[i - 1]);
				fZec4[i] = fZec0[i] * fZec3[i] / fZec2[i];
				fRec11[i] = fRec11[i - 1] + 2.0f * fZec4[i];
				fZec5[i] = fRec11[i - 1] + fZec4[i];
				fZec6[i] = fZec0[i] * fZec5[i];
				fRec12[i] = fRec12[i - 1] + 2.0f * fZec6[i];
				fRec13[i] = fRec12[i - 1] + fZec6[i];
				fZec7[i] = fZec3[i] / fZec2[i];
				fRec14[i] = fZec7[i];
				fRec15[i] = fZec5[i];
			}
			/* Post code */
			for (int j9 = 0; j9 < 4; j9 = j9 + 1) {
				fRec11_perm[j9] = fRec11_tmp[vsize + j9];
			}
			for (int j11 = 0; j11 < 4; j11 = j11 + 1) {
				fRec12_perm[j11] = fRec12_tmp[vsize + j11];
			}
			/* Vectorizable loop 15 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec10[i] = fZec8[i] * fZec9[i] + 1.0f;
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec12[i] = std::sqrt(fZec11[i]);
			}
			/* Recursive loop 17 */
			/* Pre code */
			for (int j20 = 0; j20 < 4; j20 = j20 + 1) {
				fRec20_tmp[j20] = fRec20_perm[j20];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec20[i] = fSlow6 + fConst2 * fRec20[i - 1];
			}
			/* Post code */
			for (int j21 = 0; j21 < 4; j21 = j21 + 1) {
				fRec20_perm[j21] = fRec20_tmp[vsize + j21];
			}
			/* Vectorizable loop 18 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec19[i] = 1.0f / fRec21[i] + fZec18[i];
			}
			/* Recursive loop 19 */
			/* Pre code */
			for (int j28 = 0; j28 < 4; j28 = j28 + 1) {
				fRec22_tmp[j28] = fRec22_perm[j28];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fRec22[i] = fSlow8 + fConst2 * fRec22[i - 1];
			}
			/* Post code */
			for (int j29 = 0; j29 < 4; j29 = j29 + 1) {
				fRec22_perm[j29] = fRec22_tmp[vsize + j29];
			}
			/* Recursive loop 20 */
			/* Pre code */
			for (int j30 = 0; j30 < 4; j30 = j30 + 1) {
				fRec32_tmp[j30] = fRec32_perm[j30];
			}
			for (int j32 = 0; j32 < 4; j32 = j32 + 1) {
				fRec33_tmp[j32] = fRec33_perm[j32];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec30[i] = static_cast<float>(input1[i]) - (fZec1[i] * fRec32[i - 1] + fRec33[i - 1]);
				fZec31[i] = fZec0[i] * fZec30[i] / fZec2[i];
				fRec32[i] = fRec32[i - 1] + 2.0f * fZec31[i];
				fZec32[i] = fRec32[i - 1] + fZec31[i];
				fZec33[i] = fZec0[i] * fZec32[i];
				fRec33[i] = fRec33[i - 1] + 2.0f * fZec33[i];
				fRec34[i] = fRec33[i - 1] + fZec33[i];
				fZec34[i] = fZec30[i] / fZec2[i];
				fRec35[i] = fZec34[i];
				fRec36[i] = fZec32[i];
			}
			/* Post code */
			for (int j31 = 0; j31 < 4; j31 = j31 + 1) {
				fRec32_perm[j31] = fRec32_tmp[vsize + j31];
			}
			for (int j33 = 0; j33 < 4; j33 = j33 + 1) {
				fRec33_perm[j33] = fRec33_tmp[vsize + j33];
			}
			/* Recursive loop 21 */
			/* Pre code */
			for (int j16 = 0; j16 < 4; j16 = j16 + 1) {
				fRec6_tmp[j16] = fRec6_perm[j16];
			}
			for (int j18 = 0; j18 < 4; j18 = j18 + 1) {
				fRec7_tmp[j18] = fRec7_perm[j18];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec13[i] = fRec14[i] + fRec13[i] * fZec11[i] + fRec15[i] * fZec12[i] / fRec17[i] - (fZec9[i] * fRec6[i - 1] + fRec7[i - 1]);
				fZec14[i] = fZec8[i] * fZec13[i] / fZec10[i];
				fRec6[i] = fRec6[i - 1] + 2.0f * fZec14[i];
				fZec15[i] = fRec6[i - 1] + fZec14[i];
				fZec16[i] = fZec8[i] * fZec15[i];
				fRec7[i] = fRec7[i - 1] + 2.0f * fZec16[i];
				fRec8[i] = fZec15[i];
				fZec17[i] = fZec13[i] / fZec10[i];
				fRec9[i] = fZec16[i] + fRec7[i - 1] + fZec17[i];
			}
			/* Post code */
			for (int j17 = 0; j17 < 4; j17 = j17 + 1) {
				fRec6_perm[j17] = fRec6_tmp[vsize + j17];
			}
			for (int j19 = 0; j19 < 4; j19 = j19 + 1) {
				fRec7_perm[j19] = fRec7_tmp[vsize + j19];
			}
			/* Vectorizable loop 22 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec20[i] = fZec18[i] * fZec19[i] + 1.0f;
			}
			/* Vectorizable loop 23 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec21[i] = std::pow(1e+01f, 0.05f * fRec20[i]);
			}
			/* Vectorizable loop 24 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec27[i] = std::pow(1e+01f, 0.05f * fRec22[i]);
			}
			/* Recursive loop 25 */
			/* Pre code */
			for (int j34 = 0; j34 < 4; j34 = j34 + 1) {
				fRec28_tmp[j34] = fRec28_perm[j34];
			}
			for (int j36 = 0; j36 < 4; j36 = j36 + 1) {
				fRec29_tmp[j36] = fRec29_perm[j36];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec35[i] = fRec35[i] + fRec34[i] * fZec11[i] + fRec36[i] * fZec12[i] / fRec17[i] - (fZec9[i] * fRec28[i - 1] + fRec29[i - 1]);
				fZec36[i] = fZec8[i] * fZec35[i] / fZec10[i];
				fRec28[i] = fRec28[i - 1] + 2.0f * fZec36[i];
				fZec37[i] = fRec28[i - 1] + fZec36[i];
				fZec38[i] = fZec8[i] * fZec37[i];
				fRec29[i] = fRec29[i - 1] + 2.0f * fZec38[i];
				fRec30[i] = fZec37[i];
				fZec39[i] = fZec35[i] / fZec10[i];
				fRec31[i] = fZec38[i] + fRec29[i - 1] + fZec39[i];
			}
			/* Post code */
			for (int j35 = 0; j35 < 4; j35 = j35 + 1) {
				fRec28_perm[j35] = fRec28_tmp[vsize + j35];
			}
			for (int j37 = 0; j37 < 4; j37 = j37 + 1) {
				fRec29_perm[j37] = fRec29_tmp[vsize + j37];
			}
			/* Recursive loop 26 */
			/* Pre code */
			for (int j24 = 0; j24 < 4; j24 = j24 + 1) {
				fRec0_tmp[j24] = fRec0_perm[j24];
			}
			for (int j26 = 0; j26 < 4; j26 = j26 + 1) {
				fRec1_tmp[j26] = fRec1_perm[j26];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec22[i] = fRec9[i] + fRec8[i] * fZec21[i] - (fZec19[i] * fRec0[i - 1] + fRec1[i - 1]);
				fZec23[i] = fZec18[i] * fZec22[i] / fZec20[i];
				fRec0[i] = fRec0[i - 1] + 2.0f * fZec23[i];
				fZec24[i] = fRec0[i - 1] + fZec23[i];
				fZec25[i] = fZec18[i] * fZec24[i];
				fRec1[i] = fRec1[i - 1] + 2.0f * fZec25[i];
				fRec2[i] = fRec1[i - 1] + fZec25[i];
				fZec26[i] = fZec22[i] / fZec20[i];
				fRec3[i] = fZec26[i];
				fRec4[i] = fZec24[i];
			}
			/* Post code */
			for (int j25 = 0; j25 < 4; j25 = j25 + 1) {
				fRec0_perm[j25] = fRec0_tmp[vsize + j25];
			}
			for (int j27 = 0; j27 < 4; j27 = j27 + 1) {
				fRec1_perm[j27] = fRec1_tmp[vsize + j27];
			}
			/* Vectorizable loop 27 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec28[i] = std::sqrt(fZec27[i]);
			}
			/* Recursive loop 28 */
			/* Pre code */
			for (int j38 = 0; j38 < 4; j38 = j38 + 1) {
				fRec23_tmp[j38] = fRec23_perm[j38];
			}
			for (int j40 = 0; j40 < 4; j40 = j40 + 1) {
				fRec24_tmp[j40] = fRec24_perm[j40];
			}
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec40[i] = fRec31[i] + fRec30[i] * fZec21[i] - (fZec19[i] * fRec23[i - 1] + fRec24[i - 1]);
				fZec41[i] = fZec18[i] * fZec40[i] / fZec20[i];
				fRec23[i] = fRec23[i - 1] + 2.0f * fZec41[i];
				fZec42[i] = fRec23[i - 1] + fZec41[i];
				fZec43[i] = fZec18[i] * fZec42[i];
				fRec24[i] = fRec24[i - 1] + 2.0f * fZec43[i];
				fRec25[i] = fRec24[i - 1] + fZec43[i];
				fZec44[i] = fZec40[i] / fZec20[i];
				fRec26[i] = fZec44[i];
				fRec27[i] = fZec42[i];
			}
			/* Post code */
			for (int j39 = 0; j39 < 4; j39 = j39 + 1) {
				fRec23_perm[j39] = fRec23_tmp[vsize + j39];
			}
			for (int j41 = 0; j41 < 4; j41 = j41 + 1) {
				fRec24_perm[j41] = fRec24_tmp[vsize + j41];
			}
			/* Vectorizable loop 29 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec29[i] = fRec2[i] + fRec3[i] * fZec27[i] + fRec4[i] * fZec28[i] / fRec21[i];
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				fZec45[i] = fRec25[i] + fRec26[i] * fZec27[i] + fRec27[i] * fZec28[i] / fRec21[i];
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output0[i] = static_cast<FAUSTFLOAT>(fSlow9 * fZec29[i] + fSlow10 * tanhf(fZec29[i]));
			}
			/* Vectorizable loop 32 */
			/* Compute code */
			for (int i = 0; i < vsize; i = i + 1) {
				output1[i] = static_cast<FAUSTFLOAT>(fSlow9 * fZec45[i] + fSlow10 * tanhf(fZec45[i]));
			}
		}
	}

};

#endif
