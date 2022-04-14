// Copyright 2018-2022 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Contains `MessageSenderFactory` trait.

use crate::error::InternalError;

use super::{FullyQualifiedServiceId, MessageSender};

/// Creates new `MessageSender` instances.
///
/// Implementations of `MessageSenderFactory` takes one generic for the type of message the sender
/// takes.
pub trait MessageSenderFactory<M>: Send {
    /// Returns a new `MessageSender`
    fn new_message_sender(
        &self,
        scope: &FullyQualifiedServiceId,
    ) -> Result<Box<dyn MessageSender<M>>, InternalError>;

    fn clone_boxed(&self) -> Box<dyn MessageSenderFactory<M>>;
}

impl<M> Clone for Box<dyn MessageSenderFactory<M>> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}
