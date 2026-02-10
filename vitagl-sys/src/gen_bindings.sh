VITASDK=/usr/local/vitasdk

bindgen "$VITASDK/arm-vita-eabi/include/vitaGL.h" \
  -o bindings.rs \
  -- \
  --target=arm-vita-eabi \
  --sysroot="$VITASDK" \
  -I"$VITASDK/arm-vita-eabi/include"