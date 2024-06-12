# Development

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind css cli: <https://tailwindcss.com/docs/installation>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to <http://localhost:8080>

## Rules

1. Always return only one `rsx!` per component. Conditionals should be done **within** the `rsx!` macro.
2. Never put a hook within another hook structure.
3. Always initialize hooks, even if unused, at the beginning of a page component.
4. Never instantiate hooks within components other than pages, always set them through parameters.

If you don't follow these rules, the website may crash silently and become unusable, which is a real pain in the a** to debug.

### Repo Organization

- [src/assets](src/assets): Only SVG or inline pictures components should be defined here.
- [src/components](src/components): There are the components that will be used across pages. No hook or signal should be used directly within those components.
- [src/pages](src/pages): These are all the pages of the website. They initialize hooks, assemble components and give them the required context and help navigate across routes.
- [src/routes](src/routes): All the different URLs endpoints of the application. Each route is implemented with a page.
- [src/services](src/services): These are the API calls to the backend. It is the only place where HTTP calls are allowed.
- [src/models](src/models): These are the data types used across the frontend.
- [src/errors](src/errors): All the error types of the frontend. Used to easily get and display errors sent from the backend.
- [src/hooks](src/hooks): These are ways to store and share information on the client's side. Only complex objects such as the user authentication state should be implemented here. For simple, per-page hooks such as a counter, use `use_state` directly.
- [src/constants](src/constants): Just constants stored here as a way to factorize and label code. Do not put anything executable here, only constants.

### Hooks

All hooks can implement an interface over a model. However, to access the hooks, they should provide a `use_XXX` function that returns a `Signal` to that model. For example:

```rs
/// Get the user stored information if logged in.
pub fn use_user_context() -> Signal<UseUserContextHandle> {
    let inner: UserInfoStorage = use_persistent(USER_INFO_KEY, || None);
    let navigator = navigator();

    use_signal(|| UseUserContextHandle { inner, navigator })
}
```

That way, client code can handle the `Signal` themselves, which makes using Dioxus states easier.
