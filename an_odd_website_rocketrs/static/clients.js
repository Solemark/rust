/**
 * @typedef Client
 * @type {object}
 * @property {number} id - client id
 * @property {string} first_name - client first name
 * @property {string} last_name - client last name
 * @property {string} email_address - client email address
 * @property {boolean} visible - is the client visible
 */

/**
 * On load function
 * @returns {void}
 */
const getClients = () => {
    fetch("/data/clients")
        .then(async (res) => {
            /** @type {Client[]} */
            const data = await res.json()
            updateTable(data)
        })
        .catch((err) => {
            console.log(err)
        })
}

/**
 * Updates the client table
 * @param {Client[]} data - array of client data
 * @returns {void}
 */
const updateTable = (data) => {
    let tbody = document.getElementById("client-table")
    data.forEach(client => {
        let row = document.createElement("tr")
        let id = document.createElement("td")
        let fn = document.createElement("td")
        let ln = document.createElement("td")
        let ea = document.createElement("td")
        id.innerHTML = client.id
        fn.innerHTML = client.first_name
        ln.innerHTML = client.last_name
        ea.innerHTML = client.email_address
        row.append(id, fn, ln, ea)
        tbody.append(row)
    })
}

/**
 * Clear the modal inputs
 * @returns {void}
 */
const clearModal = () => {
    document.getElementById("input-id").value = ""
    document.getElementById("input-fn").value = ""
    document.getElementById("input-ln").value = ""
    document.getElementById("input-ea").value = ""
}
