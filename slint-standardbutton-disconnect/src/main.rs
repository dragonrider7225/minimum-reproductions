slint::slint! {
    import { Button, StandardButton } from "std-widgets.slint";

    export component BrokenDialog inherits Dialog {
        callback ok_clicked();

        Text {
            text: "Broken: This Ok button doesn't work";
        }

        StandardButton { kind: ok; }
    }

    export component FunctionalDialog inherits Dialog {
        Text {
            text: "Functional: This Ok button works";
        }

        StandardButton { kind: ok; }
    }
}

macro_rules! mk_dialog_new {
    ($($type:ty),*) => {$(
        impl $type {
            fn real_new() -> Result<Self, slint::PlatformError> {
                let this = Self::new()?;
                let this_weak = this.as_weak();
                this.on_ok_clicked(move || {
                    eprintln!("Ok clicked");
                    this_weak.clone().upgrade().unwrap().hide().unwrap()
                });
                Ok(this)
            }
        }
    )*};
}

mk_dialog_new!(BrokenDialog, FunctionalDialog);

fn main() {
    let broken = BrokenDialog::real_new().unwrap();
    broken.run().unwrap();
    let functional = FunctionalDialog::real_new().unwrap();
    functional.run().unwrap();
}
