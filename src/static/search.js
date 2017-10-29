const search = document.getElementById("search-input");
const form = document.getElementById("search-form");

if(search) {
    form.action = "/projects/search/" + (search.value ? search.value : " ");

    search.addEventListener("input", (event) => {
        form.action = "/projects/search/" + (search.value ? search.value : " ");
    }, false);
}
