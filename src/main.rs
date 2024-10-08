use std::process::{ExitCode, Termination};

fn main() -> Result<Emotion, Impossible> {
    let mut me = Me {
        state: Emotion::Panic
    };

    // From the dark and desperate, I must rise up
    me = me.rise_up();

    /*
    My hope may not shimmer like a diamond,
    I may hate myself,
    though what is for sure,
    but my persistance to perfect;

    start();

    My perfection which causes burnout so,
    may it be fueled by hearty maple,
    may it shimmer and glow,
    my kindling give way to something stable;
    */

    let mut persistant = true;

    while persistant {
        let _x = match me
            .clone()
            .try_one() {
                Result::Ok(_) => break,
                Result::Err(x) => x
        };

        // Spring of will own

        let _x = match me
            .clone()
            .try_two(){
                Result::Ok(_) => break,
                Result::Err(x) => x
        };

        // Summer's final chance

        me = match me.try_final() {
            Result::Ok(x) => x,
            Result::Err(_) => panic!("Gone, gone are the coals which gave me life.")
        };

        persistant = false
    };

    /*
    Seasons know their persistant cycle,
    give each other the will to live,
    give care to group survival,
    seasons thrive;

    The cycle will endure,
    each error corrected,
    each hole patched;

    Let Autumn and Winter clean the parasites;
    Let Spring bring rejuvenation;
    Let Summer be my contribution and thanks;
    */

    // Let the cycle keep me
    Ok(Emotion::Safe)
    // At last
}

#[derive(Clone)]
struct Me {
    state: Emotion
}

impl Me {
    fn rise_up(mut self) -> Self {
        /*
        wait...,
        but how do I do that?...,
        for I am
        */
    
        self.state;
    
        /*
        I must believe,
        [I know this sounds like a cat poster, but it's true],
        I must be
        */
    
        self.state = Emotion::Hopeful;

        self
    }

    fn try_one(mut self) -> Result<(), &'static str> {
        /*
        Autumn come too late,
        must fell those branches of parasite ridden leaves,
        must outlast that cold great,
        Autumn leads, upheaves;
        */

        self.state = Emotion::Desperate;

        /*
        Winter come,
        by blind hand snowstorm cast,
        by mind numb,
        Winter snow blast;
        */

        Err("Rage, Rage against the dying of my flame;")
    }

    fn try_two(mut self) -> Result<(), &'static str> {
        self.state = Emotion::Depressed;

        /*
        Spring brings remorse,
        melt away the snowmen sank,
        melt away all ice forts in course,
        Spring's flowers, rivers drank;
        */

        Err("By charcoal hot, I can rise;")
    }

    fn try_final(mut self) -> Result<Self, ()> {
        /*
        Spring's cleaning not wasted,
        clean out the closet,
        clean grief once basted,
        Spring's soul a prophet;
        */

        self.state = Emotion::Hopeful;

        /*
        Summer flowers,
        showing their petals to the gleaming sun,
        showing their strength against the showers,
        Summer begun;
        */

        self.state = Emotion::Happy;

        Ok(self)
    }
}

#[derive(Clone)]
enum Emotion {
    Panic,
    Hopeful,
    Desperate,
    Depressed,
    Happy,
    Safe
}

impl Termination for Emotion {
    fn report(self) -> std::process::ExitCode {
        ExitCode::SUCCESS
    }
}

#[derive(Debug)]
struct Impossible;

impl Termination for Impossible {
    fn report(self) -> std::process::ExitCode {
        // I cannot fail now.
        ExitCode::SUCCESS
    }
}