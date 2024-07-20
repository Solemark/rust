/**
 * @typedef Setting
 * @type {object}
 * @property {string} name - settings name
 * @property {bool} status - settings status
 */

const getSettings = () => {
    fetch("/data/settings")
        .then(async (res) => {
            /** @type {Setting[]} */
            const data = await res.json()
            updateTable(data)
        })
        .catch((err) => {
            console.log(err)
        })
}

/**
 * update the setting table
 * @param {Setting[]} data - array of setting data
 * @returns {void} 
 */
const updateTable = (data) => {
    let tbody = document.getElementById("setting-table")
    data.forEach(setting => {
        let row = document.createElement("tr")
        let name = document.createElement("td")
        name.innerHTML = setting.name

        let status = document.createElement("input")
        status.checked = setting.status

        status.setAttribute("type", "checkbox")
        status.setAttribute("onclick", `sendSetting(${JSON.stringify(setting)})`)

        row.append(name, status)
        tbody.append(row)
    });
}

/**
 * send a post request to update the setting
 * @param {string} setting 
 */
const sendSetting = (setting) => {
    const form = document.createElement("form")
    form.method = "POST"
    form.action = "/data/settings/update"

    let name = document.createElement("input")
    name.type = "hidden"
    name.name = "name"
    name.value = setting.name

    let status = document.createElement("input")
    status.type = "hidden"
    status.name = "status"
    status.value = setting.status

    form.appendChild(name)
    form.appendChild(status)
    document.body.appendChild(form)
    form.submit()
}
