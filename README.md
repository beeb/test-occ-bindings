I copied the contents of https://github.com/Open-Cascade-SAS/OCCT/tree/master/src into the `occt` folder.

When doing `cargo run`, there are lots of errors related to `undefined reference`s and also

```
some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
```

Full logs:

```
error: linking with `cc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="[redacted]" VSLANG="1033" "cc" "-m64" "/tmp/rustcQv5EE2/symbols.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.294g3d0qcrvn7dtb.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.2a1nl96hly95e5ht.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.2k6d70khmu19wgur.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.2p0h5i3az0es54m8.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.2r649bp2fpnb73jy.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.38ps841t5su59byb.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.3lpf149manlqxk1y.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.3okt0pai47eg8f3a.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.43h5ld02gt275fj8.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.4ct9u6rkgstglczh.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.4p8vt6noo56ewlxp.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.5a6i455kfs09ej5s.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.5ciyzo49uckeg2bn.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.6g68pskat2s936f.rcgu.o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a.3h4pqv17s9kwmpge.rcgu.o" "-Wl,--as-needed" "-L" ".../test-occ-bindings/target/debug/deps" "-L" ".../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out" "-L" ".../test-occ-bindings/target/debug/build/cxx-a44ecd2ddd2ed15a/out" "-L" ".../test-occ-bindings/target/debug/build/link-cplusplus-e58842013ee94ecb/out" "-L" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-locctwrapper" ".../test-occ-bindings/target/debug/deps/libcxx-40a290e23b096734.rlib" ".../test-occ-bindings/target/debug/deps/liblink_cplusplus-2cbd9ac198187802.rlib" ".../test-occ-bindings/target/debug/deps/libanyhow-8780a210f92b7f49.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f55ee4dfcef9d6c2.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-3b7b2120c59cf4cf.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-1be4bbab557ba5f9.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-818055ac265188d7.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-c76c474cd6fc2707.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-cef5ed5b7e7bc525.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-92ada7f71df4f807.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-515d5a7096b744bd.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-9988794e1a662dfa.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-b57230631749f36f.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-55f9432d7fd1ddf0.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-67b2e0904cc8c6bf.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-e3574c40ac08e8ec.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-fd674a7ef4c5e3dc.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4fad884586188abd.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bc8a4a55f03c7704.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-639fe4992aa3175d.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-e0ad026a086e3293.rlib" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-87185c5e58e44fea.rlib" "-Wl,-Bdynamic" "-lstdc++" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" ".../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" ".../test-occ-bindings/target/debug/deps/test_occ_bindings-cbc8529af58d128a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs"
  = note: /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `Standard_Transient::operator new(unsigned long)':
          .../test-occ-bindings/./occt/Standard/Standard_Transient.hxx:35: undefined reference to `Standard::Allocate(unsigned long)'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `Standard_Transient::operator delete(void*)':
          .../test-occ-bindings/./occt/Standard/Standard_Transient.hxx:35: undefined reference to `Standard::Free(void*)'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `Standard_Transient::Standard_Transient()':
          .../test-occ-bindings/./occt/Standard/Standard_Transient.hxx:40: undefined reference to `vtable for Standard_Transient'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `Standard_Transient::~Standard_Transient()':
          .../test-occ-bindings/./occt/Standard/Standard_Transient.hxx:49: undefined reference to `vtable for Standard_Transient'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `NCollection_BaseSequence::operator delete(void*)':
          .../test-occ-bindings/./occt/NCollection/NCollection_BaseSequence.hxx:53: undefined reference to `Standard::Free(void*)'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `NCollection_BaseList::operator delete(void*)':
          .../test-occ-bindings/./occt/NCollection/NCollection_BaseList.hxx:40: undefined reference to `Standard::Free(void*)'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `NCollection_BaseList::NCollection_BaseList(opencascade::handle<NCollection_BaseAllocator> const&)':
          .../test-occ-bindings/./occt/NCollection/NCollection_BaseList.hxx:127: undefined reference to `NCollection_BaseAllocator::CommonBaseAllocator()'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `TopoDS_TShape::TopoDS_TShape()':
          .../test-occ-bindings/./occt/TopoDS/TopoDS_TShape.hxx:151: undefined reference to `vtable for TopoDS_TShape'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `TopoDS_Shape::TopoDS_Shape()':
          .../test-occ-bindings/./occt/TopoDS/TopoDS_Shape.hxx:49: undefined reference to `TopLoc_Location::TopLoc_Location()'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `Standard_Mutex::Sentry::Lock()':
          .../test-occ-bindings/./occt/Standard/Standard_Mutex.hxx:113: undefined reference to `Standard_Mutex::Lock()'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `XSControl_Reader::operator delete(void*)':
          .../test-occ-bindings/./occt/XSControl/XSControl_Reader.hxx:75: undefined reference to `Standard::Free(void*)'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `XSControl_Reader::~XSControl_Reader()':
          .../test-occ-bindings/./occt/XSControl/XSControl_Reader.hxx:92: undefined reference to `vtable for XSControl_Reader'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `STEPControl_Reader::operator delete(void*)':
          .../test-occ-bindings/./occt/STEPControl/STEPControl_Reader.hxx:74: undefined reference to `Standard::Free(void*)'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `TopoDS_TCompound::TopoDS_TCompound()':
          .../test-occ-bindings/./occt/TopoDS/TopoDS_TCompound.lxx:22: undefined reference to `vtable for TopoDS_TCompound'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `TopoDS_Builder::MakeCompound(TopoDS_Compound&) const':
          .../test-occ-bindings/./occt/TopoDS/TopoDS_Builder.lxx:83: undefined reference to `TopoDS_Builder::MakeShape(TopoDS_Shape&, opencascade::handle<TopoDS_TShape> const&) const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `MyTest::convert_step_to_stl(rust::cxxbridge1::String, rust::cxxbridge1::String)':
          .../test-occ-bindings/src/OCCTWrapper.cpp:15: undefined reference to `STEPControl_Reader::STEPControl_Reader()'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:21: undefined reference to `XSControl_Reader::ReadFile(char const*)'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:28: undefined reference to `XSControl_Reader::PrintCheckLoad(bool, IFSelect_PrintCount) const'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:30: undefined reference to `STEPControl_Reader::NbRootsForTransfer()'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:31: undefined reference to `XSControl_Reader::PrintCheckTransfer(bool, IFSelect_PrintCount) const'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:35: undefined reference to `STEPControl_Reader::TransferRoot(int, Message_ProgressRange const&)'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:36: undefined reference to `XSControl_Reader::NbShapes() const'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:41: undefined reference to `XSControl_Reader::Shape(int) const'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:42: undefined reference to `TopoDS_Builder::Add(TopoDS_Shape&, TopoDS_Shape const&) const'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:47: undefined reference to `StlAPI_Writer::StlAPI_Writer()'
          /usr/bin/ld: .../test-occ-bindings/src/OCCTWrapper.cpp:48: undefined reference to `StlAPI_Writer::Write(TopoDS_Shape const&, char const*, Message_ProgressRange const&)'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<NCollection_BaseAllocator>::EndScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:393: undefined reference to `Standard_Transient::DecrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<TopLoc_SListNodeOfItemLocation>::EndScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:393: undefined reference to `Standard_Transient::DecrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `NCollection_List<TopoDS_Shape>::Clear(opencascade::handle<NCollection_BaseAllocator> const&)':
          .../test-occ-bindings/./occt/NCollection/NCollection_List.hxx:98: undefined reference to `NCollection_BaseList::PClear(void (*)(NCollection_ListNode*, opencascade::handle<NCollection_BaseAllocator>&))'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<TopoDS_TShape>::EndScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:393: undefined reference to `Standard_Transient::DecrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `NCollection_Sequence<opencascade::handle<Standard_Transient> >::Clear(opencascade::handle<NCollection_BaseAllocator> const&)':
          .../test-occ-bindings/./occt/NCollection/NCollection_Sequence.hxx:170: undefined reference to `NCollection_BaseSequence::ClearSeq(void (*)(NCollection_SeqNode*, opencascade::handle<NCollection_BaseAllocator>&))'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<XSControl_WorkSession>::EndScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:393: undefined reference to `Standard_Transient::DecrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `NCollection_Sequence<TopoDS_Shape>::Clear(opencascade::handle<NCollection_BaseAllocator> const&)':
          .../test-occ-bindings/./occt/NCollection/NCollection_Sequence.hxx:170: undefined reference to `NCollection_BaseSequence::ClearSeq(void (*)(NCollection_SeqNode*, opencascade::handle<NCollection_BaseAllocator>&))'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<TopoDS_TCompound>::BeginScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:387: undefined reference to `Standard_Transient::IncrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<TopoDS_TCompound>::EndScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:393: undefined reference to `Standard_Transient::DecrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<NCollection_BaseAllocator>::BeginScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:387: undefined reference to `Standard_Transient::IncrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `opencascade::handle<Standard_Transient>::EndScope()':
          .../test-occ-bindings/./occt/Standard/Standard_Handle.hxx:393: undefined reference to `Standard_Transient::DecrementRefCounter() const'
          /usr/bin/ld: .../test-occ-bindings/target/debug/build/test-occ-bindings-7c96b6a35e08860a/out/libocctwrapper.a(OCCTWrapper.o): in function `STEPControl_Reader::~STEPControl_Reader()':
          .../test-occ-bindings/./occt/STEPControl/STEPControl_Reader.hxx:70: undefined reference to `vtable for STEPControl_Reader'
          collect2: error: ld returned 1 exit status

  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

error: could not compile `test-occ-bindings` due to previous error
```
