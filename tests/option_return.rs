use forward_methods::fwd;

struct Mover;
struct RefMover;
struct MutMover;

struct Borrower;
struct RefBorrower;
struct MutBorrower;

struct MoverWrapper(Mover);
struct RefMoverWrapper(RefMover);
struct MutMoverWrapper(MutMover);

struct BorrowerWrapper(Borrower);
struct RefBorrowerWrapper(RefBorrower);
struct MutBorrowerWrapper(MutBorrower);

static mut OPT: Option<String> = None;
static mut OPT_REF: Option<&String> = None;
static mut OPT_MUT: Option<&mut String> = None;

impl Mover {
    fn get_opt(self) -> Option<String> {
        None
    }
    fn get_opt_ref(self) -> Option<&'static String> {
        None
    }
    fn get_opt_mut_ref(self) -> Option<&'static mut String> {
        None
    }
}

impl<'a> RefMover {
    fn get_opt(self) -> &'a Option<String> {
        &None
    }
    fn get_opt_ref(self) -> &'a Option<&'a String> {
        &None
    }
    fn get_opt_mut_ref(self) -> &'a Option<&'a mut String> {
        &None
    }
}

impl MutMover {
    fn get_opt(self) -> &'static mut Option<String> {
        unsafe { &mut OPT }
    }
    fn get_opt_ref(self) -> &'static mut Option<&'static String> {
        unsafe { &mut OPT_REF }
    }
    fn get_opt_mut_ref(self) -> &'static mut Option<&'static mut String> {
        unsafe { &mut OPT_MUT }
    }
}

impl Borrower {
    fn get_opt(&self) -> Option<String> {
        None
    }
    fn get_opt_ref(&self) -> Option<&'static String> {
        None
    }
    fn get_opt_mut_ref(&self) -> Option<&'static mut String> {
        None
    }
}

impl<'a> RefBorrower {
    fn get_opt(&self) -> &'a Option<String> {
        &None
    }
    fn get_opt_ref(&self) -> &'a Option<&'a String> {
        &None
    }
    fn get_opt_mut_ref(&self) -> &'a Option<&'a mut String> {
        &None
    }
}

impl<'a> MutBorrower {
    fn get_opt(&mut self) -> &'a mut Option<String> {
        unsafe { &mut OPT }
    }
    fn get_opt_ref(&mut self) -> &'a mut Option<&'static String> {
        unsafe { &mut OPT_REF }
    }
    fn get_opt_mut_ref(&mut self) -> &'a mut Option<&'static mut String> {
        unsafe { &mut OPT_MUT }
    }
}

impl MoverWrapper {
    fwd!(
        fn get_opt(self) -> Option<String>,
        fn get_opt_ref(self) -> Option<&'static String>,
        fn get_opt_mut_ref(self) -> Option<&'static mut String>
        to self.0
    );
}

impl<'a> RefMoverWrapper {
    fwd!(
        fn get_opt(self) -> &'a Option<String>,
        fn get_opt_ref(self) -> &'a Option<&'a String>,
        fn get_opt_mut_ref(self) -> &'a Option<&'a mut String>
        to self.0
    );
}

impl MutMoverWrapper {
    fwd!(
        fn get_opt(self) -> &'static mut Option<String>,
        fn get_opt_ref(self) -> &'static mut Option<&'static String>,
        fn get_opt_mut_ref(self) -> &'static mut Option<&'static mut String>
        to self.0
    );
}

impl<'a> BorrowerWrapper {
    fwd!(
        fn get_opt(&self) -> Option<String>,
        fn get_opt_ref(&self) -> Option<&'a String>,
        fn get_opt_mut_ref(&self) -> Option<&'a mut String>
        to self.0
    );
}

impl<'a> RefBorrowerWrapper {
    fwd!(
        fn get_opt(&self) -> &'a Option<String>,
        fn get_opt_ref(&self) -> &'a Option<&'a String>,
        fn get_opt_mut_ref(&self) -> &'a Option<&'a mut String>
        to self.0
    );
}

impl<'a> MutBorrowerWrapper {
    fwd!(
        fn get_opt(&mut self) -> &'a mut Option<String>,
        fn get_opt_ref(&mut self) -> &'a mut Option<&'static String>,
        fn get_opt_mut_ref(&mut self) -> &'a mut Option<&'static mut String>
        to self.0
    );
}

#[test]
fn should_forward_methods_from_mover() {
    assert!(MoverWrapper(Mover).get_opt().is_none());
    assert!(MoverWrapper(Mover).get_opt_ref().is_none());
    assert!(MoverWrapper(Mover).get_opt_mut_ref().is_none());
}

#[test]
fn should_forward_methods_from_ref_mover() {
    assert!(RefMoverWrapper(RefMover).get_opt().is_none());
    assert!(RefMoverWrapper(RefMover).get_opt_ref().is_none());
    assert!(RefMoverWrapper(RefMover).get_opt_mut_ref().is_none());
}

#[test]
fn should_forward_methods_from_mut_mover() {
    assert!(MutMoverWrapper(MutMover).get_opt().is_none());
    assert!(MutMoverWrapper(MutMover).get_opt_ref().is_none());
    assert!(MutMoverWrapper(MutMover).get_opt_mut_ref().is_none());
}

#[test]
fn should_forward_methods_from_borrower() {
    let cmp = BorrowerWrapper(Borrower);

    assert!(cmp.get_opt().is_none());
    assert!(cmp.get_opt_ref().is_none());
    assert!(cmp.get_opt_mut_ref().is_none());
}

#[test]
fn should_forward_methods_from_ref_borrower() {
    let cmp = RefBorrowerWrapper(RefBorrower);

    assert!(cmp.get_opt().is_none());
    assert!(cmp.get_opt_ref().is_none());
    assert!(cmp.get_opt_mut_ref().is_none());
}

#[test]
fn should_forward_methods_from_mut_borrower() {
    let mut cmp = MutBorrowerWrapper(MutBorrower);

    assert!(cmp.get_opt().is_none());
    assert!(cmp.get_opt_ref().is_none());
    assert!(cmp.get_opt_mut_ref().is_none());
}
