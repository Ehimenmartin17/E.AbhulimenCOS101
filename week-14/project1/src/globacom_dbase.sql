--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email character varying(20) NOT NULL,
    c_mobile bigint NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(8) NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: employees; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employees (
    eid integer NOT NULL,
    name text NOT NULL,
    dno integer NOT NULL,
    esal integer NOT NULL,
    age integer NOT NULL,
    phone bigint NOT NULL
);


ALTER TABLE public.employees OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname character(1) NOT NULL,
    pduration text NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	MUSTA KARIM	35	M_KARIM@GMAIL.COM	8055089112	102	5
111	LILIAN JAIYA	43	L_JAIYE@GMAIL.COM	8055185341	100	3
113	PHILIPAKONJO	41	P_AKONJ@GMAIL.COM	9052356772	100	2
117	OKAFOR MATHIAS	45	O_MATHIAS@GMAIL.COM	8056763367	120	10
118	SAMSON ADELEKE	65	S_ADELEKE@GMAIL.COM	7056774423	117	11
119	LAWAL TAMIRE	35	L_TAMIRE@GMAIL.COM	9052111101	107	5
112	ARTHUR MUSA	50	A_MUSA@GMAIL.COM	9052356772	100	2
114	MARYLENE MAPA	33	M_MAPA@GMAIL.COM	8053333551	120	5
115	OGHENERO AGOR	50	O_AGOR@GMAIL.COM	8056765424	117	11
116	ADAMS BREE	33	A_BREE@GMAIL.COM	8056765424	102	1
120	JAMES JOB	44	J_JOB@GMAIL.COM	8059693919	100	8
121	MATTHEW JAKANDE	21	M_JAKANDE@GMAIL.COM	7051232144	120	2
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	ADMINISTRATION	IKEJA	44
101	2	ACCOUNT	EGBEDA	11
100	3	PACKAGING	AJAH	44
120	4	RESEARCH	VICTORIA ISLAND	33
97	5	ACCOUNT	MAGODO	22
122	6	OPERATIONS	MILE 2	44
107	7	PACKAGING	KETU	55
\.


--
-- Data for Name: employees; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employees (eid, name, dno, esal, age, phone) FROM stdin;
101	ALADE JOY	2	250000	33	8023089832
100	MUSTAPHA ALI	3	175000	32	8063285831
107	ALAKWE MARTIN	7	380000	48	7090082812
97	DANKADE AMINAT	5	550000	40	9023688832
108	JOSIAH JOSHUA	1	120000	30	8053189131
102	ADELEKE JANE	4	200000	38	7061045862
120	MAKINDE MARY	2	450000	55	9023487830
122	OSAHON MARK	6	320000	44	802228942
117	SULEMAN AJAYI	3	800000	50	7030089811
104	KUTI LAWAL	1	750000	35	9145689842
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 MONTHS	102
22	B	14 MONTHS	97
33	C	16 MONTHS	120
44	D	25 MONTHS	108
55	E	9 MONTHS	107
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: employees employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (eid);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

