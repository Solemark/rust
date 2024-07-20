/**
 * @typedef Job
 * @type {object}
 * @property {number} id - job id
 * @property {string} name - job name
 * @property {string} created - job created timestamp
 * @property {string} status - job status
 *
 * @typedef Export
 * @type {object}
 * @property {number} id - job id
 * @property {string} name - job name
 */

/**
 * On load function
 * @returns {void}
 */
const getJobs = () => {
    fetch("/data/jobs")
        .then(async (res) => {
            /** @type {Job[]} */
            const data = await res.json()
            updateTable(data)
        })
        .catch((err) => {
            console.log(err)
        })

    fetch("/data/exports")
        .then(async (res) => {
            /** @type {Export[]} */
            const data = await res.json()
            updateExports(data)
        })
        .catch((err) => {
            console.log(err)
        })
}

/**
 * Updates the job table
 * @param {Job[]} data = array of job data
 * @returns {void}
 */
const updateTable = (data) => {
    let tbody = document.getElementById("jobs-table")
    data.forEach(job => {
        let row = document.createElement("tr")
        let id = document.createElement("td")
        id.innerHTML = job.id
        let name = document.createElement("td")
        name.innerHTML = job.name
        let created = document.createElement("td")
        created.innerHTML = job.created
        let status = document.createElement("td")
        status.innerHTML = job.status

        row.append(id, name, created, status)
        tbody.append(row)
    })
}

/**
 * Updates the export table
 * @param {Export[]} data = array of export data
 * @returns {void}
 */
const updateExports = (data) => {
    let select = document.getElementById("export-select")
    data.forEach(e => {
        let option = document.createElement("option")
        option.value = e.id
        option.text = e.name
        select.append(option)
    })
}

/**
 * Create a new Job and Form post
 * @returns {void}
 */
const sendJob = () => {
    /** @type {Job[]} */
    let data = []
    fetch("/data/jobs")
        .then(async (res) => {
            data = await res.json()
        })
        .catch((err) => {
            console.log(err)
        })

    let select = document.getElementById("export-select")

    const form = document.createElement("form")
    form.method = "POST"
    form.action = "/data/jobs/new"

    let id = document.createElement("input")
    id.type = "hidden"
    id.name = "id"
    id.value = data.length

    let name = document.createElement("input")
    name.type = "hidden"
    name.name = "name"
    name.value = select[select.value].text

    let created = document.createElement("input")
    created.type = "hidden"
    created.name = "created"
    created.value = new Date().toString()

    let status = document.createElement("input")
    status.type = "hidden"
    status.name = "status"
    status.value = "Pending"

    form.appendChild(id)
    form.appendChild(name)
    form.appendChild(status)
    form.appendChild(created)
    document.body.appendChild(form)
    form.submit()
}
