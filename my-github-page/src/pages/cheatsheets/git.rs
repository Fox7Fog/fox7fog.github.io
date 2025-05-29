use leptos::*;

#[component]
pub fn GitCheatsheet() -> impl IntoView {
    view! {
        <div class="prose prose-invert max-w-none">
            <h1 class="text-4xl font-bold mb-8 text-white">"Git Commands"</h1>

            <div class="space-y-8">
                <CheatsheetSection
                    title="Basic Operations"
                    commands=vec![
                        ("Clone repository", "git clone <url>", Some("git clone https://github.com/user/repo.git")),
                        ("Check status", "git status", None),
                        ("Add files", "git add <file>", Some("git add . # Add all files")),
                        ("Commit changes", "git commit -m \"message\"", None),
                    ]
                />

                <CheatsheetSection
                    title="Branching"
                    commands=vec![
                        ("Create branch", "git checkout -b <branch>", Some("git checkout -b feature/new-feature")),
                        ("Switch branch", "git checkout <branch>", None),
                        ("List branches", "git branch -a", None),
                        ("Delete branch", "git branch -d <branch>", None),
                    ]
                />

                <CheatsheetSection
                    title="Advanced Operations"
                    commands=vec![
                        ("Interactive rebase", "git rebase -i HEAD~<n>", Some("git rebase -i HEAD~3")),
                        ("Force push safely", "git push --force-with-lease", None),
                        ("Stash with message", "git stash push -m \"<message>\"", None),
                        ("Cherry pick", "git cherry-pick <commit>", None),
                    ]
                />
            </div>
        </div>
    }
}

#[component]
fn CheatsheetSection(
    title: &'static str,
    commands: Vec<(&'static str, &'static str, Option<&'static str>)>,
) -> impl IntoView {
    view! {
        <section class="bg-gray-800 rounded-lg p-6 border border-gray-700">
            <h2 class="text-2xl font-semibold mb-4 text-blue-400">{title}</h2>
            <div class="space-y-4">
                {commands.into_iter().map(|(desc, cmd, example)| view! {
                    <CommandItem description=desc command=cmd example=example/>
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn CommandItem(
    description: &'static str,
    command: &'static str,
    example: Option<&'static str>,
) -> impl IntoView {
    let (copied, set_copied) = create_signal(false);

    let copy_command = move |cmd: String| {
        // Copy to clipboard logic would go here
        set_copied.set(true);
        set_timeout(
            move || set_copied.set(false),
            std::time::Duration::from_millis(2000),
        );
    };

    view! {
        <div class="border-l-4 border-blue-500 pl-4">
            <div class="flex items-center justify-between mb-2">
                <h3 class="font-medium text-gray-200">{description}</h3>
                <button
                    class="px-2 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded transition-colors"
                    on:click=move |_| copy_command(command.to_string())
                >
                    {move || if copied.get() { "Copied!" } else { "Copy" }}
                </button>
            </div>
            <code class="block bg-gray-900 p-3 rounded text-green-400 font-mono text-sm">
                {command}
            </code>
            {example.map(|ex| view! {
                <div class="mt-2 text-sm text-gray-400">
                    "Example: " <code class="bg-gray-700 px-1 rounded">{ex}</code>
                </div>
            })}
        </div>
    }
}
