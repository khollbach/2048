; I don't know what any of this means, but it "fixes" the linker errors.
; The downside: code that's supposed to panic gets miscompiled instead.

;define void @_ZN4core9panicking5panic17hf53fd8b0bfa5848eE() {
;start:
;  ret void
;}
;
;define void @_ZN4core9panicking9panic_fmt17hffb879d9ea45e31bE() {
;start:
;  ret void
;}
;
;define void @_ZN4core9panicking18panic_bounds_check17h5bba910f5c349feaE() {
;start:
;  ret void
;}
