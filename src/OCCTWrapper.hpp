
#ifndef occtwrapper_OCCTWrapper_hpp_
#define occtwrapper_OCCTWrapper_hpp_

#include "rust/cxx.h"

namespace MyTest
{

    bool convert_step_to_stl(rust::String step_file_path, rust::String stl_file_path);

}; // namespace MyTest

#endif // occtwrapper_OCCTWrapper_hpp_