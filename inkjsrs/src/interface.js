let STORY;

function init_ink(ink) {
	STORY = new inkjs.Story(JSON.parse(ink))
}

function compile(source) {
	return new inkjs.Compiler(source).Compile().ToJson();
}

function canContinue() {
	return STORY.canContinue;
}

function continueOnce() {
	return STORY.Continue()
}

function continueMaximally() {
	let output = [];
	while (canContinue()) {
		output.push(continueOnce())
		STORY.TryFollowDefaultInvisibleChoice()
	}
	return output;
}

function save() {
	return STORY.state.ToJson();
}

function load(json) {
	STORY.state.LoadJson(json);
}

function current_choices() {
	let out = []
	for (const choice of STORY.currentChoices ) {
		out.push({text: choice.text, index: choice.index, tags: choice.tags || []})
	}
	return out
}

function choose(index) {
	STORY.ChooseChoiceIndex(index)
}