// RGB standard library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use core::ops::Neg;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use lnpbp::rgb::schema::{
    script, AssignmentAction, Bits, DataFormat, DiscreteFiniteFieldFormat, GenesisSchema,
    Occurences, Schema, StateFormat, StateSchema, TransitionSchema,
};

use crate::error::ServiceErrorDomain;
use crate::type_map;

#[derive(Debug, Display, Error, From)]
#[display_from(Debug)]
pub enum SchemaError {
    #[derive_from(core::option::NoneError)]
    NotAllFieldsPresent,

    WrongSchemaId,
}

impl From<SchemaError> for ServiceErrorDomain {
    fn from(err: SchemaError) -> Self {
        ServiceErrorDomain::Schema(format!("{}", err))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, ToPrimitive, FromPrimitive)]
#[display_from(Debug)]
#[repr(u16)]
pub enum FieldType {
    Ticker = 0,
    Name = 1,
    Description = 2,
    TotalSupply = 3,
    IssuedSupply = 4,
    DustLimit = 5,
    Precision = 6,
    PruneProof = 7,
    Timestamp = 8,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, ToPrimitive, FromPrimitive)]
#[display_from(Debug)]
#[repr(u16)]
pub enum AssignmentsType {
    Issue = 0,
    Assets = 1,
    Prune = 2,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display, ToPrimitive, FromPrimitive)]
#[display_from(Debug)]
#[repr(u16)]
pub enum TransitionType {
    Issue = 0,
    Transfer = 1,
    Prune = 2,
}

pub fn schema() -> Schema {
    Schema {
        field_types: type_map! {
            FieldType::Ticker => DataFormat::String(16),
            FieldType::Name => DataFormat::String(256),
            FieldType::Description => DataFormat::String(1024),
            FieldType::TotalSupply => DataFormat::Unsigned(Bits::Bit64, 0, core::u64::MAX as u128),
            FieldType::Precision => DataFormat::Unsigned(Bits::Bit8, 0, 18u128),
            FieldType::IssuedSupply => DataFormat::Unsigned(Bits::Bit64, 0, core::u64::MAX as u128),
            FieldType::PruneProof => DataFormat::Bytes(core::u16::MAX),
            // While UNIX timestamps allow negative numbers; in context of RGB Schema, assets
            // can't be issued in the past before RGB or Bitcoin even existed; so we prohibit
            // all the dates before RGB release
            // TODO: Update lower limit with the first RGB release
            // Current lower time limit is 07/04/2020 @ 1:54pm (UTC)
            FieldType::Timestamp => DataFormat::Integer(Bits::Bit64, 1593870844, core::i64::MAX as i128)
        },
        assignment_types: type_map! {
            AssignmentsType::Issue => StateSchema {
                format: StateFormat::Declarative,
                abi: bmap! {
                    AssignmentAction::Validate => script::Procedure::Standard(script::StandardProcedure::IssueControl)
                }
            },
            AssignmentsType::Assets => StateSchema {
                format: StateFormat::DiscreteFiniteField(DiscreteFiniteFieldFormat::Unsigned64bit),
                abi: bmap! {
                    AssignmentAction::Validate => script::Procedure::Standard(script::StandardProcedure::ConfidentialAmount)
                }
            },
            AssignmentsType::Epoch => StateSchema {
                format: StateFormat::Declarative,
                abi: bmap! {
                    AssignmentAction::Validate => script::Procedure::None
                }
            },
            AssignmentsType::Replacement => StateSchema {
                format: StateFormat::Declarative,
                abi: bmap! {
                    AssignmentAction::Validate => script::Procedure::Standard(script::StandardProcedure::Replacement)
                }
            }
        },
        genesis: GenesisSchema {
            metadata: type_map! {
                FieldType::Ticker => Occurences::Once,
                FieldType::Name => Occurences::Once,
                FieldType::Description => Occurences::NoneOrOnce,
                FieldType::TotalSupply => Occurences::Once,
                FieldType::IssuedSupply => Occurences::Once,
                FieldType::DustLimit => Occurences::NoneOrOnce,
                FieldType::Precision => Occurences::Once,
                FieldType::Timestamp => Occurences::Once
            },
            defines: type_map! {
                AssignmentsType::Issue => Occurences::NoneOrOnce,
                AssignmentsType::Epoch => Occurences::NoneOrOnce,
                AssignmentsType::Assets => Occurences::NoneOrUpTo(None),
            },
            abi: bmap! {},
        },
        transitions: type_map! {
            TransitionType::Issue => TransitionSchema {
                metadata: type_map! {
                    FieldType::IssuedSupply => Occurences::Once
                },
                closes: type_map! {
                    AssignmentsType::Issue => Occurences::Once
                },
                defines: type_map! {
                    AssignmentsType::Issue => Occurences::NoneOrOnce,
                    AssignmentsType::Epoch => Occurences::NoneOrOnce
                    AssignmentsType::Assets => Occurences::NoneOrUpTo(None)
                },
                abi: bmap! {}
            },
            TransitionType::Transfer => TransitionSchema {
                metadata: type_map! {},
                closes: type_map! {
                    AssignmentsType::Assets => Occurences::OnceOrUpTo(None)
                },
                defines: type_map! {
                    AssignmentsType::Assets => Occurences::NoneOrUpTo(None)
                },
                abi: bmap! {}
            },
            TransitionType::Epoch => TransitionSchema {
                metadata: type_map! {},
                closes: type_map! {
                    AssignmentsType::Epoch => Occurences::Once
                },
                defines: type_map! {
                    AssignmentsType::Epoch => Occurences::NoneOrOnce,
                    AssignmentsType::Replacement => Occurences::NoneOrOnce,
                },
                abi: bmap! {}
            },
            TransitionType::Replacement => TransitionSchema {
                metadata: type_map! {
                    FieldType::IssuedSupply => Occurences::Once,
                    FieldType::PruneProof => Occurences::NoneOrUpTo(None)
                },
                closes: type_map! {
                    AssignmentsType::Replacement => Occurences::Once
                },
                defines: type_map! {
                    AssignmentsType::Replacement => Occurences::NoneOrOnce,
                    AssignmentsType::Assets => Occurences::OnceOrUpTo(None)
                },
                abi: bmap! {}
            }
        },
    }
}

impl Neg for FieldType {
    type Output = usize;

    fn neg(self) -> Self::Output {
        self.to_usize().unwrap()
    }
}

impl Neg for AssignmentsType {
    type Output = usize;

    fn neg(self) -> Self::Output {
        self.to_usize().unwrap()
    }
}

impl Neg for TransitionType {
    type Output = usize;

    fn neg(self) -> Self::Output {
        self.to_usize().unwrap()
    }
}
