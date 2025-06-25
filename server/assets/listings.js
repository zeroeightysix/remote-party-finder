(function () {
    let stateWasNull = false;

    const state = {
        allowed: [],
        centre: 'All',
        roles: 0n,
        list: null,
        lang: null,
    };

    function addJsClass() {
        document.children[0].className = 'js';
    }

    function saveLoadState() {
        let saved = localStorage.getItem('state');
        if (saved !== null) {
            try {
                saved = JSON.parse(saved, (key, value) => key === 'roles' ?
                    BigInt(value) : value);
                if (!Array.isArray(saved.allowed)) {
                    saved = {};
                    stateWasNull = true;
                }
            } catch (e) {
                saved = {};
                stateWasNull = true;
            }

            for (let key in saved) {
                state[key] = saved[key];
            }
        } else {
            stateWasNull = true;
        }

        window.addEventListener('pagehide', () => {
            let copy = {};
            for (let key in state) {
                if (key === 'list') {
                    continue;
                }

                copy[key] = state[key];
            }

            localStorage.setItem('state', JSON.stringify(copy, (_, value) =>
                typeof value === 'bigint' ? value.toString() : value));
        });
    }

    function reflectState() {
        let category = document.getElementById('category-filter');
        for (let option of category.options) {
            if (stateWasNull) {
                console.log('was null');
                state.allowed.push(option.value);
            }

            option.selected = state.allowed.includes(option.value);
        }

        let dataCentre = document.getElementById('data-centre-filter');
        dataCentre.value = state.centre;

        if (stateWasNull || state.roles <= 0n) {
            state.roles = 0n;
        } else {
            let roleFilterInputs = document.getElementById('role-filter')
                .getElementsByTagName('input');
            let newRolesState = 0n;
            for (let input of roleFilterInputs) {
                let value = BigInt(input.value);
                if (state.roles & value) {
                    input.checked = true;
                    newRolesState |= value;
                }
            }
            // In case any unnecessary bits were set
            state.roles = newRolesState;
        }

        let language = document.getElementById('language');
        if (state.lang === null) {
            state.lang = language.dataset.accept;
        }

        let cookie = document.cookie
                             .split(';')
                             .find(row => row.trim().startsWith('lang='));
        if (cookie !== undefined) {
            state.lang = decodeURIComponent(cookie.split('=')[1]);
        }
    }

    function setUpList() {
        let options = {
            valueNames: [
                'duty',
                'creator',
                'description',
                {data: ['centre']},
            ],
        };
        return new List('container', options);
    }

    function refilter() {
        function categoryFilter(item) {
            let category = item.elm.dataset.pfCategory;

            return category === 'unknown' || state.allowed.includes(category);
        }

        function dataCentreFilter(item) {
            return state.centre === "All" || state.centre === item.values().centre;
        }

        function roleFilter(item) {
            return state.roles === 0n || state.roles & BigInt(item.elm.dataset.joinableRoles);
        }

        state.list.filter(item => dataCentreFilter(item) && categoryFilter(item) && roleFilter(item));
    }

    function setUpDataCentreFilter() {
        let select = document.getElementById('data-centre-filter');

        let dataCentres = {};
        for (let elem of document.querySelectorAll('#listings > .listing')) {
            let centre = elem.dataset['centre'];
            if (!dataCentres.hasOwnProperty(centre)) {
                dataCentres[centre] = 0;
            }

            dataCentres[centre] += 1;
        }

        for (let opt of select.options) {
            let centre = opt.value;

            let count = 0;

            if (dataCentres.hasOwnProperty(centre)) {
                count = dataCentres[centre];
            }

            if (centre === 'All') {
                count = Object.values(dataCentres).reduce((a, b) => a + b, 0);
            }

            opt.innerText += ` (${count})`;
        }

        select.addEventListener('change', () => {
            state.centre = select.value;
            refilter();
        });
    }

    function setUpCategoryFilter() {
        let select = document.getElementById('category-filter');

        select.addEventListener('change', () => {
            let allowed = [];

            for (let option of select.options) {
                if (!option.selected) {
                    continue;
                }

                let category = option.value;
                allowed.push(category);
            }

            state.allowed = allowed;
            refilter();
        });
    }

    function setUpRoleFilter() {
        let select = document.getElementById('role-filter');

        select.addEventListener('change', (event) => {
            let value = BigInt(event.target.value);
            if (event.target.checked) {
                state.roles |= value;
            } else {
                state.roles &= ~value;
            }
            refilter();
        });
    }

    addJsClass();
    saveLoadState();
    reflectState();
    state.list = setUpList();
    setUpDataCentreFilter();
    setUpCategoryFilter();
    setUpRoleFilter();
    refilter();
})();
