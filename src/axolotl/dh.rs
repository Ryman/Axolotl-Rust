pub trait DH {
	type Private : Clone;
	type Public : Clone;
	type Shared;

	fn public(key : &Self::Private) -> Self::Public;
	fn shared(mine : &Self::Private, theirs : &Self::Public) -> Self::Shared;
}


pub struct DHKeyPair<T> where T:DH {
	pub key : T::Private,
	pub public : T::Public,
}

impl <T:DH> Clone for DHKeyPair<T> {
	fn clone(&self) -> Self {
		DHKeyPair {
			key : self.key.clone(),
			public : self.public.clone(),
		}
	}
}

pub struct DHExchangedPair<T> where T:DH {
	pub mine : T::Private,
	pub theirs : T::Public,
}