// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

module rooch_examples::blog_updated {

    use rooch_examples::blog::{Self, BlogUpdated};
    use std::string::String;

    public fun name(blog_updated: &BlogUpdated): String {
        blog::blog_updated_name(blog_updated)
    }
   
}
