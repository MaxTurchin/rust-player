/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class application_RustApi */

#ifndef _Included_application_RustApi
#define _Included_application_RustApi
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     application_RustApi
 * Method:    rInit
 * Signature: (Lapplication/RustApi;)[J
 */
JNIEXPORT jlongArray JNICALL Java_application_RustApi_rInit
  (JNIEnv *, jclass, jobject);

/*
 * Class:     application_RustApi
 * Method:    rPlayOrPause
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_application_RustApi_rPlayOrPause
  (JNIEnv *, jclass, jlong);

/*
 * Class:     application_RustApi
 * Method:    rAddToQueue
 * Signature: (JLjava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_application_RustApi_rAddToQueue
  (JNIEnv *, jclass, jlong, jstring);

#ifdef __cplusplus
}
#endif
#endif
