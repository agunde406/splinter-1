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

//! Contains `TimerHandlerFactory` trait.

use crate::error::InternalError;

use super::TimerHandler;

/// Used to create new `TimerHandler` instances.
pub trait TimerHandlerFactory: Send {
    type Message;

    /// Returns a new `TimerHandler`
    fn new_handler(&self) -> Result<Box<dyn TimerHandler<Message = Self::Message>>, InternalError>;

    fn clone_box(&self) -> Box<dyn TimerHandlerFactory<Message = Self::Message>>;
}
